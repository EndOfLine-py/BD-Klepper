<script setup>
import {ref} from 'vue';
import {invoke} from "@tauri-apps/api/core";
import {downloadDir, join} from '@tauri-apps/api/path';

const textarea_content = ref('');

const isWorking = ref(false);
const mediaFormat = ref('mp4');
const linkCount = ref(0);

const sleep = (ms) => new Promise(resolve => setTimeout(resolve, ms));

function change_format() {
  const nextFormat = {
    'mp4': 'mp3',
    'mp3': 'wav',
    'wav': 'ogg',
    'ogg': 'mp4'
  };

  mediaFormat.value = nextFormat[mediaFormat.value] || 'mp4';
}

function reformat_textarea(event) {
  const textarea = event.target;
  const originalText = textarea_content.value;

  // Regex breakdown:
  // (?<!^|\n) -> Negative lookbehind: Ensure the link is NOT at the start of the text or start of a line
  // (https?:\/\/[^\s]+) -> Matches http:// or https:// followed by non-whitespace characters
  const NewlineUrlRegex = /(?<!^|\n)(https?:\/\/[^\s]+)/g;
  const ValidUrlRegex =  /(https?:\/\/(?:www\.|(?!www))[a-zA-Z0-9][a-zA-Z0-9-]+[a-zA-Z0-9]\.[^\s]{2,}|www\.[a-zA-Z0-9][a-zA-Z0-9-]+[a-zA-Z0-9]\.[^\s]{2,}|https?:\/\/(?:www\.|(?!www))[a-zA-Z0-9]+\.[^\s]{2,}|www\.[a-zA-Z0-9]+\.[^\s]{2,})/g;

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
</script>

<template>
  <div class="container">
    <div class="linkbox">
      <div class="textarea-wrapper">
        <textarea draggable="false" id="textarea" name="textarea" @input="reformat_textarea" v-model="textarea_content" translate="no" autocorrect="off" spellcheck="false"></textarea>
      </div>
    </div>
    <div class="controls">
      <button id="formatchange" @click="change_format">{{ mediaFormat }}</button>
      <button>Klep it all!</button>
      <div class="status-wrapper">
        <p class="status">{{ linkCount }} BD{{ linkCount > 1 ? "s" : ""}} detected</p>
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

    &::selection {
      background-color: color-mix(in srgb, var(--hack-lime) 40%, black);
    }
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
  flex-grow: 1;
  text-align: right;

 p {
  cursor: default;
  user-select: none;
  font-family: "Rajdhani Medium", serif;
  text-shadow: var(--dull) 0 0 12px;
  font-size: large;
  text-align: right;
}
}
</style>