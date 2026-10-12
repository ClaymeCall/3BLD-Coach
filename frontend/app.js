document.getElementById('scramble-btn').addEventListener('click', async () => {
  const len = document.getElementById('scramble-length').value;
  const resp = await fetch(`/api/scramble?length=${len}`);
  const data = await resp.json();
  document.getElementById('scramble-output').textContent = data.scramble;
});

document.getElementById('memo-btn').addEventListener('click', async () => {
  const resp = await fetch('/api/memo', {
    method: 'POST',
    headers: {'Content-Type': 'application/json'},
    body: JSON.stringify({})
  });
  const data = await resp.json();
  document.getElementById('memo-output').textContent = JSON.stringify(data);
});
