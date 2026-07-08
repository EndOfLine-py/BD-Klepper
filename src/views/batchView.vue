<script setup>
import {ref} from 'vue';
import {invoke} from "@tauri-apps/api/core";
import {downloadDir, join} from '@tauri-apps/api/path';

const textarea_content = ref('');

const isWorking = ref(false);
const mediaFormat = ref('mp4');
const linkCount = ref(0);
const processedLinkCount = ref(0);
const failedLinkCount = ref(0);
const processDone = ref(false);

// Regex breakdown:
// (?<!^|\n) -> Negative lookbehind: Ensure the link is NOT at the start of the text or start of a line
// (https?:\/\/[^\s]+) -> Matches http:// or https:// followed by non-whitespace characters
const NewlineUrlRegex = /(?<!^|\n)(https?:\/\/[^\s]+)/g;
const ValidUrlRegex = /(https?:\/\/(?:www\.|(?!www))[a-zA-Z0-9][a-zA-Z0-9-]+[a-zA-Z0-9]\.[^\s]{2,}|www\.[a-zA-Z0-9][a-zA-Z0-9-]+[a-zA-Z0-9]\.[^\s]{2,}|https?:\/\/(?:www\.|(?!www))[a-zA-Z0-9]+\.[^\s]{2,}|www\.[a-zA-Z0-9]+\.[^\s]{2,})/g;

function change_format() {
  const nextFormat = {
    'mp4': 'mp3',
    'mp3': 'wav',
    'wav': 'ogg',
    'ogg': 'mp4'
  };

  mediaFormat.value = nextFormat[mediaFormat.value] || 'mp4';
}

function reformat_textarea() {
  const textarea = document.getElementById('textarea');
  const originalText = textarea_content.value;

  const correctedText = originalText.replace(NewlineUrlRegex, '\n$1');

  linkCount.value = (correctedText.match(ValidUrlRegex) || []).length;

  if (correctedText !== originalText) {
    const start = textarea.selectionStart;
    const end = textarea.selectionEnd;

    textarea_content.value = correctedText;

    // Sets the cursor to the correct position
    setTimeout(() => {
      textarea.setSelectionRange(start + 1, end + 1);
    }, 0);
  }
}

async function multi_klep() {
  // THE LOGIC
  // The app has a counter that only increments on valid URLs
  // The downloader takes ALL lines in the TextArea, valid URL or not.
  // Both remaining counter and total counter are on the same value based on the number of valid URL
  // Remaining counter only decrements on URL that failed, not invalid strings URL
  // Deletes the URLs that worked, and puts back the lines that error'd or invalid.
  // Error counter only works on the valid URLs that error'd during download.
  processDone.value = false;
  if (!textarea_content.value || isWorking.value || linkCount.value === 0) {
    return;
  }

  processedLinkCount.value = linkCount.value;
  isWorking.value = true;

  let urlList = textarea_content.value.split("\n");
  let final_urlList = [];

  for (const urlListKey in urlList) {
    let url = urlList[urlListKey];

    console.log(url);
    console.log(url.match(ValidUrlRegex));

    if (url.match(ValidUrlRegex)) {
      try {
        const baseDir = await downloadDir();

        const outputPath = await join(baseDir, '%(title)s_%(id)s.%(ext)s');

        await invoke('download_single', {
          url: url,
          mediaFormat: mediaFormat.value,
          outputPath: outputPath
        });
      } catch (error) {
        final_urlList.push(url);
        failedLinkCount.value += 1;
      }
      finally {
        processedLinkCount.value -= 1;
      }
    }
    else {
      final_urlList.push(url);
    }
  }

  isWorking.value = false;
  processDone.value = true;
  textarea_content.value = final_urlList.join('\n');
  reformat_textarea();
}
</script>

<template>
  <div class="container">
    <div class="linkbox">
      <div class="textarea-wrapper">
        <textarea draggable="false" id="textarea" name="textarea" @input="reformat_textarea" v-model="textarea_content"
                  translate="no" autocorrect="off" spellcheck="false" :disabled="isWorking"></textarea>
      </div>
    </div>
    <div class="controls">
      <button id="formatchange" @click="change_format">{{ mediaFormat }}</button>
      <button @click="multi_klep" :disabled="isWorking">{{ isWorking === true ? '' : 'Klep it all!' }}</button>
      <div class="status-wrapper">
       <div v-if="!isWorking">
         <p>{{ linkCount }}</p>
         <p>BD{{ linkCount > 1 ? "s" : "" }} detected.</p>
       </div>
        <div v-if="isWorking">
          <p>{{processedLinkCount}}</p>
          <p>/</p>
          <p>{{ linkCount }}</p>
          <p>BD{{ linkCount > 1 ? "s" : "" }} left.</p>
        </div>
        <div v-if="processDone">
          <p id="error">{{ failedLinkCount }} BD{{ failedLinkCount > 1 ? "s" : "" }} Failed.</p>
        </div>
      </div>
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
}

.controls {
  display: flex;
  flex-direction: row;
  align-items: center;
  justify-content: space-between;
  width: 100%;
}

.linkbox {
  display: flex;
  flex-direction: row;
  align-items: center;
  justify-content: center;
  text-align: center;
  width: 100%;
  height: 100%;
  padding: 10px;
}

.textarea-wrapper {
  width: 100%;
  height: 100%;

  background-color: var(--hack-lime);

  clip-path: polygon(
      0 0,
      100% 0,
      100% calc(100% - 12px),
      calc(100% - 12px) 100%,
      0 100%
  );

  textarea {
    display: block;
    border: none;
    outline: none;

    background-image: linear-gradient(rgba(255, 255, 255, 0.02) 1px, transparent 1px);
    background-size: 500% 10px;

    /* Your typography and colors */
    font-family: monospace, monospace;
    font-size: 14px;
    color: var(--hack-lime);
    text-shadow: var(--hack-lime) 0 0 12px;
    background-color: var(--dullest);

    scrollbar-width: thin;
    scrollbar-color: var(--dullest) var(--hack-lime);

    margin: 1px;
    width: calc(100% - 2px);
    height: calc(100% - 2px);

    resize: none;

    box-sizing: border-box;

    clip-path: polygon(
        0 0,
        100% 0,
        100% calc(100% - 12px),
        calc(100% - 12px) 100%,
        0 100%
    );

    transition: background-color 0.2s;

    &::selection {
      background-color: color-mix(in srgb, var(--hack-lime) 40%, black);
    }

    &:disabled {
      background-color: color-mix(in srgb, var(--hack-lime) 40%, black);
    }
  }
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

  transition: width 0.2s;

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

#formatchange {
  text-transform: uppercase;

  width: 70px;
  padding: 7px;
}

.status-wrapper {
  display: flex;
  flex-grow: 1;
  flex-direction: row;
  gap: 80px;
  padding-left: 50px;
  text-align: left;
  text-wrap: nowrap;
  align-items: center;

  div {
    display: flex;
    flex-direction: row;
    gap: 10px;
    text-align: left;
    text-wrap: nowrap;
    align-items: center;
  }

  p {
    cursor: default;
    user-select: none;
    font-family: "Rajdhani Medium", serif;
    text-shadow: var(--dull) 0 0 12px;
    font-size: large;
    font-variant-numeric: tabular-nums;
  }
}

#error {
  border: var(--red) 1px solid;
  padding: 5px;
  padding-bottom: 2px;
  padding-top: 2px;
  color: var(--red);
  text-shadow: var(--red) 0 0 12px;
}
</style>