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

    const outputPath = await join(baseDir, '%(title)s.%(ext)s');

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
    'wav': 'mp4'
  };

  // Look up the current format, and set it to the next one.
  // The '|| mp4' is a failsafe just in case the value gets messed up.
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
        class="cyber-action-btn">
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
}

button:hover {
  background-color: var(--red);
}

button:active {
  background-color: var(--accent);
  color: black;
}

button::before {
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

button:hover::before {
  background-color: var(--dull);
}

button:active::before {
  background-color: var(--accent);
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

button:disabled {
  cursor: not-allowed;
  color: var(--dull);
}

button:disabled::after {
  content: '   ';
  display: inline-block;
  text-align: left;
  width: 24px;
  animation: content-cycle 1.6s infinite linear;
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
  background-color: var(--accent); /* Turns white on focus, change to whatever you like */
}

.cyber-p-wrapper {

  display: inline-block; /* Or 'block' if you want it to stretch full width */
  transition: width 0.5s;
  width: 0%;
  margin: 15px;

  background-color: var(--yellow); /* Your OUTLINE color */

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
  margin: 1px; /* The thickness of your border */
  padding: 5px;

  text-wrap: nowrap;

  font-family: "Rajdhani Medium", serif;
  font-size: 20px;
  color: var(--yellow);
  background-color: var(--dullest); /* Your inner background color */

  text-align: left;

  text-shadow: 0 0 12px var(--dull);

  /* Must have the exact same clip-path to match the wrapper shape */
  clip-path: polygon(
      0 0,
      100% 0,
      100% calc(100% - 12px),
      calc(100% - 12px) 100%,
      0 100%
  );

  user-select: none;
}

/* The converted badge styles */
.cyber-badge {
  display: inline-flex;
  align-items: center; /* Centers the '!' vertically */
  justify-content: center; /* Centers the '!' horizontally */

  width: 24px; /* Size of the circle */
  height: 24px;
  border-radius: 50%; /* Makes it a perfect circle */

  background-color: #fbc531; /* Cyberpunk warning yellow */
  color: #0c1015; /* Matches your dark UI color */

  font-family: "Arial Black", sans-serif; /* Makes the '!' nice and bold */
  font-size: 20px;

  /* Soft red glow behind the circle */
  box-shadow: 0 0 12px var(--dull);

  /* Keeps the badge from shrinking if the paragraph text gets long */
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