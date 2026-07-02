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

function change_format() {
  const nextFormat = {
    'mp4': 'mp3',
    'mp3': 'wav',
    'wav': 'ogg',
    'ogg': 'mp4'
  };

  // Look up the current format, and set it to the next one.
  // The '|| mp4' is a failsafe just in case the value gets messed up.
  mediaFormat.value = nextFormat[mediaFormat.value] || 'mp4';
}
</script>

<template>
  <textarea draggable="false"></textarea>
  <button id="formatchange" @click="change_format">{{ mediaFormat }}</button>
  <button>Klep it all!</button>
</template>

<style scoped>
textarea {
  width: 100%;
  resize: none;
}
</style>