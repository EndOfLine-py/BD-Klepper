<script setup>

</script>

<template>
<div class="container">
  <div class="cyber-input-wrapper">
    <input type="text">
  </div>
  <button>Klep it!</button>
  <p>Status</p>
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

  gap: 0.5em;
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

  top:    1px;
  left:   1px;
  right:  1px;
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

/* 1. The Wrapper acts as the border color and clips the shape */
.cyber-input-wrapper {
  position: relative;
  display: inline-block;
  width: 70%; /* Moved your width here */
  margin: 10px; /* Moved your margin here */

  background-color: var(--dull); /* This is your OUTLINE color */

  clip-path: polygon(
      0 0,
      100% 0,
      100% calc(100% - 12px),
      calc(100% - 12px) 100%,
      0 100%
  );
}

/* 2. The Input sits inside, slightly offset to reveal the wrapper's color underneath */
.cyber-input-wrapper input[type="text"] {
  display: block;
  border: none;
  outline: none;

  /* Your typography and colors */
  font-family: "Rajdhani Medium", serif;
  font-size: 20px;
  color: var(--accent);
  background-color: var(--dullest);

  /* Creates the "border thickness" by shrinking the input slightly inside the wrapper */
  margin: 1px;
  width: calc(100% - 2px);
  height: calc(100% - 2px);

  /* Padding stays on the input so typing looks correct */
  padding: 15px;
  box-sizing: border-box;

  /* Must clip the input exactly the same way so it matches the wrapper */
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

</style>