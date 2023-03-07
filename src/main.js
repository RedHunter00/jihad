const { invoke } = window.__TAURI__.tauri;

// let greetInputEl;
// let greetMsgEl;

// async function greet() {
//   Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
//   greetMsgEl.textContent = await invoke("greet", { name: greetInputEl.value });
// }

async function slot() {
  await invoke("slot_audio", {});
}

window.addEventListener("DOMContentLoaded", () => {
  const slotImages = document.querySelectorAll('.slot img');
  const spinButton = document.querySelector('#spin-button');

  const imageSources = ['slots-image1.png', 'slots-image2.png', 'daniel-pula.gif'];

  spinButton.addEventListener('click', function () {
    slot();
    
    slotImages.forEach(function (image) {
      for (let i = 0; i < 25; i++) {
        setTimeout(function () {
          let randomIndex = Math.floor(Math.random() * imageSources.length);
          image.src = imageSources[randomIndex];
        }, i * 100 + Math.random() * 256);
      }
    });
    
  });
});

