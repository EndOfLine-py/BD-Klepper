<script setup>

</script>

<template>
<div class="container">
  <div class="cyber-input-wrapper">
    <input type="text" autocomplete="off" autocapitalize="off" spellcheck="false"/>
  </div>
  <button>Klep it!</button>
  <div class="cyber-p-wrapper">
    <p>
      Status
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


.cyber-input-wrapper {
  position: relative;
  display: inline-block;
  width: 80%;

  margin-bottom: 10px;

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
  width: 50%;
  margin: 10px;
  align-content: center;

  background-color: var(--yellow); /* Your OUTLINE color */

  clip-path: polygon(
      0 0,
      100% 0,
      100% calc(100% - 12px),
      calc(100% - 12px) 100%,
      0 100%
  );
}

.cyber-p-wrapper p {
  margin: 1px; /* The thickness of your border */
  padding: 5px;

  font-family: "Rajdhani Medium", serif;
  font-size: 20px;
  color: var(--yellow);
  background-color: var(--dullest); /* Your inner background color */

  text-align: left;

  /* Must have the exact same clip-path to match the wrapper shape */
  clip-path: polygon(
      0 0,
      100% 0,
      100% calc(100% - 12px),
      calc(100% - 12px) 100%,
      0 100%
  );
}
.cyber-p-wrapper p::before {
  content: '!';                 /* The exclamation mark */
  display: inline-flex;
  align-items: center;          /* Centers the '!' vertically */
  justify-content: center;      /* Centers the '!' horizontally */

  width:  24px;                  /* Size of the circle */
  height: 24px;
  border-radius: 50%;           /* Makes it a perfect circle */

  background-color: #fbc531;    /* Cyberpunk warning yellow */
  color: #0c1015;               /* Matches your dark UI color */

  font-family: "Arial Black", sans-serif; /* Makes the '!' nice and bold */
  font-size: 20px;

  /* Optional: Soft red glow behind the circle to match the image */
  box-shadow: 0 0 12px rgba(255, 71, 87, 0.4);

  /* Keeps the badge from shrinking if the paragraph text gets long */
  flex-shrink: 0;
}
</style>