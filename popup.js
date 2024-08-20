document.getElementById('sendUrl').addEventListener('click', async () => {
  // Get the active tab
  const [tab] = await chrome.tabs.query({ active: true, currentWindow: true });

  if (tab) {
    const url = tab.url;

    // Send the URL to the localhost endpoint
    fetch('http://localhost:3000/receive-url', {
      method: 'POST',
      headers: {
        'Content-Type': 'application/json'
      },
      body: JSON.stringify({ url: url })
    })
    .then(response => response.text())
    .then(data => console.log('Success:', data))
    .catch((error) => console.error('Error:', error));
  }
});
