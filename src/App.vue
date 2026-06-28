<script setup>
import { ref } from "vue";
import { invoke } from "@tauri-apps/api/core";
import { getCurrentWindow } from '@tauri-apps/api/window';
import { openUrl } from '@tauri-apps/plugin-opener';

const greetMsg = ref("");
const name = ref("");

async function greet() {
  // Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
  greetMsg.value = await invoke("greet", { name: name.value });
}

const openGitHub = async () => {
  try {
    await openUrl('https://github.com/EndOfLine-py');
  } catch (error) {
    console.error("Failed to open link:", error);
  }
};

const appWindow = getCurrentWindow();
const minimize = async () => {
  await appWindow.minimize();
};
const close = async () => {
  await appWindow.close();
};
const loader = document.getElementsByClassName("loader")[0];
const loaderFadeOut = loader.animate(
    [
      // Keyframes
      { opacity: 1 },  // Start at 100% opacity
      { opacity: 0}   // End at 0% opacity
    ],
    {
      // Timing Options
      duration: 500,     // Animation duration in milliseconds (e.g., 500ms = 0.5s)
      easing: 'ease-out', // Smooth slow-down effect at the end
      fill: 'forwards',   // Crucial: Keeps the element at 0% opacity when finished
      delay: 500
    }
);
loaderFadeOut.onfinish = () => {
  loader.style.display = 'none';
}
</script>

<template>
  <header class="svg-underline">
    <nav>
        <router-link to="/" draggable="false" id="title">BD Klepper</router-link>
        <span>|</span>
        <router-link to="/" draggable="false">Solo Klep</router-link>
        <router-link to="/batch" draggable="false">Klep W/ Chooms</router-link>
    </nav>
    <div data-tauri-drag-region class="titlebar">
      <button @click="close" id="titlebar-close">X</button>
      <button @click="minimize" id="titlebar-minimize">_</button>
    </div>
  </header>

  <main>
    <RouterView/>
  </main>

  <footer>
    <p>
      Made with <3 by <span @click="openGitHub">EndOfLine-Py</span>
    </p>
  </footer>
</template>

<style scoped>

</style>
<style>
.router-link-exact-active {
  color: var(--accent);
  text-shadow: var(--accent) 0 0 12px;
}
.router-link-exact-active:hover {
  text-shadow: var(--accent) 0 0 12px;
}

header {
  display: flex;
  width: 100%;

  flex-direction: row;
  justify-content: space-between;

  border-bottom: var(--dull) 1px solid;
}

#title {
  padding-left: 10px;
  font-family: "Rajdhani Medium",serif;
  color: var(--red);

  text-decoration: none;
  font-size: 28px;

  cursor: default;
  text-shadow: var(--red) 0 0 16px;
}

nav {
  display: flex;
  user-select: none;

  flex-direction: row;
  align-items: center;
  justify-content: center;

  color: var(--red);

  gap: 1rem;

  span {
    color: var(--red);
  }
}

nav a:hover {
  text-shadow: var(--red) 0 0 12px;
}

footer {
  padding: 0;
  width: 100%;
  user-select: none;
  border-top: var(--dull) 1px solid;
  display: flex;
  align-content: center;
  justify-content: center;
  span {
    cursor: pointer;
    color: var(--accent);
  }
  text-shadow: var(--dull) 0 0 12px;
}

.titlebar {
  display: flex;
  align-items: center;

  flex-direction: row-reverse;
  flex-grow: 1;

  padding-right: 10px;

  button {
    margin-left: 8px;
    height: 25px;
    width:  25px;
    font-weight: bolder;
    color: var(--accent);
    border: var(--dull) 1px solid;
    outline: none;
    font-family: "Consolas", sans-serif;
    background-color: var(--dullest);
    user-select: none;
  }
  button:hover {
    background-color: var(--dull);
    border: var(--red) 1px solid;
  }
}

*[data-tauri-drag-region] {
  app-region: drag;
}

</style>
