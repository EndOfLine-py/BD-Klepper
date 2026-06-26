<script setup>
import { ref } from "vue";
import { invoke } from "@tauri-apps/api/core";
import { getCurrentWindow } from '@tauri-apps/api/window';

const greetMsg = ref("");
const name = ref("");

async function greet() {
  // Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
  greetMsg.value = await invoke("greet", { name: name.value });
}

const appWindow = getCurrentWindow();
const minimize = async () => {
  await appWindow.minimize();
};
const close = async () => {
  await appWindow.close();
};
</script>

<template>
  <header>
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

  <main class="container">
    <RouterView/>
  </main>

  <footer>
    <p>
      Made with <3 by <a href="https://github.com/EndOfLine-py">EndOfLine-Py</a>
    </p>
  </footer>
</template>

<style scoped>

</style>
<style>
.router-link-exact-active {
  color: var(--accent);
}

header {
  display: flex;
  width: 100%;

  flex-direction: row;
  justify-content: space-between;

  border-bottom: var(--text-dark-light) 1px solid;
}

#title {
  padding-left: 10px;
  font-family: "Rajdhani Medium",serif;
  color: var(--text-accent);

  text-decoration: none;
  font-size: 28px;

  cursor: default;
}

nav {
  display: flex;
  user-select: none;

  flex-direction: row;
  align-items: center;
  justify-content: center;

  color: var(--text-accent);

  gap: 1rem;

  span {
    color: var(--text-dark-light);
  }
}

footer {
  width: 100%;
  user-select: none;
  border-top: var(--text-dark-light) 1px solid;
  display: flex;
  justify-content: center;
  a {
    color: var(--accent);
  }
}

.titlebar {
  display: flex;
  flex-direction: row-reverse;
  flex-grow: 1;
  button {
    height: 25px;
    width:  25px;
    font-weight: bolder;
    color: var(--accent);
    outline: var(--text-dark) 1px solid;
    border: none;
    font-family: "Consolas", sans-serif;
    background-color: rgba(0, 0, 0, 0.1);
  }
  button:hover {
    background-color: rgba(255, 255, 255, 0.05);
    outline: var(--text-dark-light) 1px solid;
  }
}

*[data-tauri-drag-region] {
  app-region: drag;
}
</style>
