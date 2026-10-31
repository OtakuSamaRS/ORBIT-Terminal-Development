import { Terminal } from "@xterm/xterm";
import { FitAddon } from "@xterm/addon-fit";
import { invoke } from "@tauri-apps/api/core";
import { listen } from "@tauri-apps/api/event";
import "@xterm/xterm/css/xterm.css";

const atlas = document.getElementById("atlas");
const shellList = document.getElementById("shell-list");
const scanStatus = document.getElementById("scan-status");
const shellCount = document.getElementById("shell-count");
const atlasMessage = document.getElementById("atlas-message");
const terminalView = document.getElementById("terminal-view");
const terminalElement = document.getElementById("terminal");
const activeShellName = document.getElementById("active-shell-name");
const backButton = document.getElementById("back-to-atlas");

const term = new Terminal({
  cursorBlink: true,
  fontSize: 14,
  fontFamily: 'Consolas, "Cascadia Mono", "Courier New", monospace',
  scrollback: 5000,
  theme: {
    background: "#0b0d12",
    foreground: "#e8ebf2",
    cursor: "#a4f36c",
    selectionBackground: "#454b58",
  },
});

const fitAddon = new FitAddon();
term.loadAddon(fitAddon);

let terminalOpened = false;
let started = false;
let starting = false;
let outputListener = null;
let exitListener = null;
let selectedShell = null;
let resizeTimer = null;

function showAtlas(message = "") {
  atlas.hidden = false;
  terminalView.hidden = true;
  atlasMessage.textContent = message;
}

function showTerminal() {
  atlas.hidden = true;
  terminalView.hidden = false;

  if (!terminalOpened) {
    term.open(terminalElement);
    terminalOpened = true;
  }

  requestAnimationFrame(() => {
    fitAddon.fit();
    term.focus();
  });
}

function getShellName(shell) {
  return shell.name ?? shell.display_name ?? shell.executable ?? "Unknown shell";
}

function getShellExecutable(shell) {
  return shell.executable ?? shell.path ?? "";
}

function renderShells(shells) {
  shellList.replaceChildren();

  if (!Array.isArray(shells) || shells.length === 0) {
    scanStatus.textContent = "DISCOVERY COMPLETE";
    shellCount.textContent = "0 FOUND";

    const empty = document.createElement("div");
    empty.className = "loading-card";
    empty.textContent = "No supported shells were detected on this machine.";
    shellList.appendChild(empty);
    return;
  }

  scanStatus.textContent = "DISCOVERY COMPLETE";
  shellCount.textContent = `${shells.length} FOUND`;

  shells.forEach((shell, index) => {
    const name = getShellName(shell);
    const executable = getShellExecutable(shell);

    const card = document.createElement("button");
    card.type = "button";
    card.className = "shell-card";

    const top = document.createElement("div");
    top.className = "shell-top";

    const title = document.createElement("span");
    title.className = "shell-name";
    title.textContent = name;

    const arrow = document.createElement("span");
    arrow.className = "shell-arrow";
    arrow.textContent = "↗";
    arrow.setAttribute("aria-hidden", "true");

    top.append(title, arrow);

    const meta = document.createElement("div");
    meta.className = "shell-meta";
    meta.textContent = executable || "Executable path unavailable";

    card.append(top, meta);
    card.setAttribute("aria-label", `Launch ${name}`);

    card.addEventListener("click", () => {
      selectedShell = shell;
      activeShellName.textContent = name.toUpperCase();
      showTerminal();
      startTerminal();
    });

    shellList.appendChild(card);
  });
}

async function discoverShells() {
  scanStatus.textContent = "SCANNING AVAILABLE SHELLS";
  shellCount.textContent = "-- FOUND";

  try {
    const shells = await invoke("detect_shells");
    renderShells(shells);
  } catch (error) {
    scanStatus.textContent = "DISCOVERY FAILED";
    shellCount.textContent = "ERROR";

    const message = `Shell discovery failed: ${String(error)}`;
    shellList.replaceChildren();

    const errorCard = document.createElement("div");
    errorCard.className = "loading-card";
    errorCard.textContent = message;
    shellList.appendChild(errorCard);

    console.error(message);
  }
}

async function startTerminal() {
  if (started || starting || !selectedShell) return;
  starting = true;
  atlasMessage.textContent = "";

  try {
    if (!outputListener) {
      outputListener = await listen("terminal-output", (event) => {
        term.write(event.payload);
      });
    }

    if (!exitListener) {
      exitListener = await listen("terminal-exited", () => {
        started = false;
        starting = false;
        term.writeln("\r\n\x1b[90mShell session ended.\x1b[0m");
      });
    }

    fitAddon.fit();

    await invoke("start_terminal", {
      shellExecutable: getShellExecutable(selectedShell),
      cols: Math.max(1, term.cols),
      rows: Math.max(1, term.rows),
    });

    started = true;
    term.focus();
  } catch (error) {
    term.writeln("");
    term.writeln(`\x1b[31mFailed to start shell: ${String(error)}\x1b[0m`);
    console.error("Terminal startup failed:", error);

    if (outputListener) {
      outputListener();
      outputListener = null;
    }

    if (exitListener) {
      exitListener();
      exitListener = null;
    }
  } finally {
    starting = false;
  }
}

term.attachCustomKeyEventHandler((event) => {
  const isCopyKey =
    (event.ctrlKey || event.metaKey) &&
    event.key.toLowerCase() === "c";

  if (!isCopyKey) return true;

  const selection = term.getSelection();

  if (event.shiftKey || selection.length > 0) {
    if (selection.length > 0) {
      navigator.clipboard.writeText(selection).catch((error) => {
        console.error("Copy failed:", error);
      });
    }

    return false;
  }

  return true;
});

term.onData(async (data) => {
  if (!started) return;

  try {
    await invoke("write_terminal", { data });
  } catch (error) {
    term.writeln(`\r\n\x1b[31mInput error: ${String(error)}\x1b[0m`);
  }
});

backButton.addEventListener("click", async () => {
  backButton.disabled = true;

  try {
    if (started || starting) {
      await invoke("stop_terminal");
    }
  } catch (error) {
    console.error("Could not stop terminal:", error);
    term.writeln(
      `\r\n\x1b[31mCould not stop shell: ${String(error)}\x1b[0m`,
    );
    return;
  } finally {
    backButton.disabled = false;
  }

  started = false;
  starting = false;
  selectedShell = null;
  term.clear();
  showAtlas();
});

window.addEventListener("resize", () => {
  clearTimeout(resizeTimer);

  resizeTimer = setTimeout(async () => {
    if (!terminalOpened || terminalView.hidden) return;

    fitAddon.fit();

    if (started) {
      try {
        await invoke("resize_terminal", {
          cols: Math.max(1, term.cols),
          rows: Math.max(1, term.rows),
        });
      } catch (error) {
        console.error("Terminal resize failed:", error);
      }
    }
  }, 100);
});

window.addEventListener("beforeunload", () => {
  if (outputListener) outputListener();
  if (exitListener) exitListener();
});

showAtlas();
discoverShells();
