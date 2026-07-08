<script setup>
import { ref } from 'vue';
import { invoke } from "@tauri-apps/api/core";
import { downloadDir, join } from '@tauri-apps/api/path';

const videoUrl = ref('');

const isWorking = ref(false);

const statusText = ref('Nothing to report');
const statusType = ref('idle'); // Idle, Warning, Error, Success
const badgeText = ref('!');
const statusColor = ref("var(--dull)");
const messageWidth = ref('0%');
const mediaFormat = ref('mp4');

const sleep = (ms) => new Promise(resolve => setTimeout(resolve, ms));

async function single_klep() {
  if (!videoUrl.value) {
    await set_status('warning', 'Missing URL, gonk.')
    return;
  }
  isWorking.value = true;

  await set_status('warning', 'Jacked in.');

  try {

    const baseDir = await downloadDir();

    const outputPath = await join(baseDir, '%(title)s_%(id)s.%(ext)s');

    await set_status('Working...', 'Jacked in.');
    const output = await invoke('download_single', {
      url: videoUrl.value,
      mediaFormat: mediaFormat.value,
      outputPath: outputPath
    });

    isWorking.value = false;
    await set_status('success', output);
    await sleep(5000);
    await set_status('idle', "");
  } catch (error) {
    isWorking.value = false;
    await set_status('error', error);
  }
}

function change_format() {
  const nextFormat = {
    'mp4': 'mp3',
    'mp3': 'wav',
    'wav': 'ogg',
    'ogg': 'mp4'
  };

  mediaFormat.value = nextFormat[mediaFormat.value] || 'mp4';
}

let statusSequenceId = 0;

async function set_status(type, status_text) {
  statusSequenceId++;
  const currentInstanceId = statusSequenceId;

  switch (type) {
    case 'idle':
      statusType.value = 'idle';
      badgeText.value = '';
      statusColor.value = 'var(--dull)';
      messageWidth.value = '0%';
      break;
    case 'warning':
      statusType.value = 'warning';
      badgeText.value = '!';
      statusColor.value = 'var(--yellow)';
      messageWidth.value = '60%';
      break;
    case 'error':
      statusType.value = 'error';
      badgeText.value = 'X';
      statusColor.value = 'var(--red)';
      messageWidth.value = '60%';
      break;
    case 'success':
      statusType.value = 'success';
      badgeText.value = '✔';
      statusColor.value = 'var(--green)';
      messageWidth.value = '60%';
      break;
  }

  statusText.value = status_text;

  await sleep(2000);

  if (currentInstanceId === statusSequenceId) {
    messageWidth.value = '0%';
  }

  await sleep(505);
}
</script>

<template>
  <div class="container">

    <div class="main-input-wrapper">
      <button id="formatchange" @click="change_format">{{ mediaFormat }}</button>
      <div class="cyber-input-wrapper">
        <input v-model="videoUrl" type="text" autocomplete="off" autocapitalize="off" spellcheck="false"/>
      </div>
    </div>

    <button
        :disabled=isWorking
        @click="single_klep"
        class="cyber-action-btn"
    >
      {{ isWorking === true ? '' : 'Klep it !' }}
    </button>

    <div class="cyber-p-wrapper" :style="{backgroundColor: statusColor, width: messageWidth}">
      <p :style="{color: statusColor}">
        <span class="cyber-badge" :style="{backgroundColor: statusColor}">{{ badgeText }}</span>
        {{ statusText }}
      </p>
    </div>
  </div>

</template>

<style scoped>
.container {
  display: flex;

  flex-direction: column;

  align-items: center;
  justify-content: center;
  text-align: center;

  width: 100%;
  height: 100%;

  margin-top: 10px;

}

@keyframes content-cycle {
  0%, 24% {
    content: '   ';
  }
  25%, 49% {
    content: '.  ';
  }
  50%, 74% {
    content: '.. ';
  }
  75%, 100% {
    content: '...';
  }
}
button {
  position: relative;
  display: inline-block;

  padding: 7px 40px;
  border: none;
  margin: 10px;

  font-family: "Rajdhani Medium", serif;
  font-size: 20px;

  color: var(--accent);

  background-color: var(--dull);
  clip-path: polygon(
      0 0,
      100% 0,
      100% calc(100% - 12px),
      calc(100% - 12px) 100%,
      0 100%
  );

  cursor: pointer;
  user-select: none;

  &:hover {
    background-color: var(--red);
  }

  &:active {
    background-color: var(--accent);
    color: black;
  }

  &::before {
    content: '';
    position: absolute;

    top: 1px;
    left: 1px;
    right: 1px;
    bottom: 1px;

    background-color: var(--dullest);

    z-index: -1;

    clip-path: polygon(
        0 0,
        100% 0,
        100% calc(100% - 12px),
        calc(100% - 12px) 100%,
        0 100%
    );
  }

  &:hover::before {
    background-color: var(--dull);
  }

  &:active::before {
    background-color: var(--accent);
  }

  &:disabled {
    cursor: not-allowed;
    color: var(--dull);
  }

  &:disabled::after {
    content: '   ';
    display: inline-block;
    text-align: left;
    width: 24px;
    animation: content-cycle 1.6s infinite linear;
  }
}

.cyber-input-wrapper {
  width: 80%;

  background-color: var(--dull);

  clip-path: polygon(
      0 0,
      100% 0,
      100% calc(100% - 12px),
      calc(100% - 12px) 100%,
      0 100%
  );
}


.cyber-input-wrapper input[type="text"] {
  display: block;
  border: none;
  outline: none;

  /* Your typography and colors */
  font-family: "Rajdhani Medium", serif;
  font-size: 20px;
  color: var(--accent);
  text-shadow: var(--accent) 0 0 12px;
  background-color: var(--dullest);

  margin: 1px;
  width: calc(100% - 2px);
  height: calc(100% - 2px);

  padding: 15px;
  box-sizing: border-box;

  clip-path: polygon(
      0 0,
      100% 0,
      100% calc(100% - 12px),
      calc(100% - 12px) 100%,
      0 100%
  );
}

.cyber-input-wrapper:focus-within {
  background-color: var(--accent);
}

.cyber-p-wrapper {

  display: inline-block;
  transition: width 0.5s;
  width: 0%;
  margin: 15px;

  background-color: var(--yellow);

  clip-path: polygon(
      0 0,
      100% 0,
      100% calc(100% - 12px),
      calc(100% - 12px) 100%,
      0 100%
  );

  user-select: none;
  box-shadow: 0 0 12px var(--dull);
  overflow: hidden;
}

.cyber-p-wrapper p {
  margin: 1px;
  padding: 5px;

  text-wrap: nowrap;

  font-family: "Rajdhani Medium", serif;
  font-size: 20px;
  color: var(--yellow);
  background-color: var(--dullest);

  text-align: left;

  text-shadow: 0 0 12px var(--dull);


  clip-path: polygon(
      0 0,
      100% 0,
      100% calc(100% - 12px),
      calc(100% - 12px) 100%,
      0 100%
  );

  user-select: none;
}


.cyber-badge {
  display: inline-flex;
  align-items: center;
  justify-content: center;

  width: 24px;
  height: 24px;
  border-radius: 50%;

  background-color: #fbc531;
  color: #0c1015;

  font-family: "Arial Black", sans-serif;
  font-size: 20px;

  box-shadow: 0 0 12px var(--dull);

  flex-shrink: 0;

  margin-right: 10px;

  user-select: none;
}

#formatchange {
  text-transform: uppercase;

  width: 70px;
  padding: 7px;
}

.main-input-wrapper {
  display: flex;
  flex-direction: row;
  align-items: center;
  flex-wrap: nowrap;
  width: 100%;
  margin-top: 50px;
}
</style>