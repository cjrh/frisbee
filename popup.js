let log = (msg) => {
  // document.getElementById('msgs').textContent += msg;
  // console.log(msg);
}

document.getElementById('sendUrl').addEventListener('click', async () => {
  log('inside button click');
  const [tab] = await browser.tabs.query({ active: true, currentWindow: true });
  log(`tab: ${tab}`);

  if (tab) {
    const url = tab.url;
    log(`url: ${url}`);

    // Send the URL to the localhost endpoint
    fetch('http://localhost:5800', {
      method: 'POST',
      headers: {
        'Content-Type': 'application/json'
      },
      body: JSON.stringify({ url: url })
    })
    .then(response => response.text())
    .then(data => log(`Success: ${data}`))
    .catch((error) => log(`Error: ${error}`));
  }
});
