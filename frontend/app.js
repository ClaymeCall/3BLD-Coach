document.getElementById('scramble-btn').addEventListener('click', async () => {
  const btn = document.getElementById('scramble-btn');
  const spinner = document.getElementById('scramble-spinner');
  const output = document.getElementById('scramble-output');
  const errDiv = document.getElementById('scramble-error');
  btn.disabled = true; spinner.style.display = 'inline-block'; errDiv.textContent = '';
  try {
    const len = document.getElementById('scramble-length').value;
    const resp = await fetch(`/api/scramble?length=${len}`);
    if (!resp.ok) throw new Error(`HTTP ${resp.status}`);
    const data = await resp.json();
    output.textContent = data.scramble;
  } catch (e) {
    errDiv.textContent = 'Failed to fetch scramble: ' + e.message;
    output.textContent = '';
  } finally {
    btn.disabled = false; spinner.style.display = 'none';
  }
});

document.getElementById('memo-start-btn').addEventListener('click', async () => {
  const btn = document.getElementById('memo-start-btn');
  const spinner = document.getElementById('memo-spinner');
  const output = document.getElementById('memo-output');
  const errDiv = document.getElementById('memo-error');
  const methodSelect = document.getElementById('memo-method');
  btn.disabled = true; spinner.style.display = 'inline-block'; errDiv.textContent = '';
  try {
    const method = methodSelect.value;
    const resp = await fetch('/api/memo', {
      method: 'POST',
      headers: {'Content-Type': 'application/json'},
      body: JSON.stringify({ method })
    });
    if (!resp.ok) throw new Error(`HTTP ${resp.status}`);
    const data = await resp.json();
    output.textContent = JSON.stringify(data, null, 2);
  } catch (e) {
    errDiv.textContent = 'Failed to start memo: ' + e.message;
    output.textContent = '';
  } finally {
    btn.disabled = false; spinner.style.display = 'none';
  }
});
