import { LitElement, html, css } from 'https://unpkg.com/lit@2.8.0?module';

class ScrambleApp extends LitElement {
  static styles = css`
    .spinner {border:4px solid #f3f3f3;border-top:4px solid #3498db;border-radius:50%;width:20px;height:20px;animation:spin 1s linear infinite;display:inline-block;vertical-align:middle;}
    @keyframes spin {to{transform:rotate(360deg);}}
    .error {color:#c00; margin-top:0.5rem;}
    .output {background:#eee;padding:1rem;white-space:pre-wrap;}
    label {display:block;margin-bottom:0.5rem;}
    button {margin-top:0.5rem;}
  `;
  static properties = {
    length: {type: Number},
    loading: {type: Boolean},
    scramble: {type: String},
    error: {type: String},
  };
  constructor() {
    super();
    this.length = 25;
    this.loading = false;
    this.scramble = '';
    this.error = '';
  }
  render() {
    return html`
      <h2>Generate Scramble</h2>
      <label>Length: <input type="number" .value=${this.length} min="1" max="100" @input=${e=>this.length = Number(e.target.value)}></label>
      <button ?disabled=${this.loading} @click=${this._fetch}>Generate</button>
      ${this.loading ? html`<span class="spinner"></span>` : ''}
      ${this.error ? html`<div class="error">${this.error}</div>` : ''}
      ${this.scramble ? html`<pre class="output">${this.scramble}</pre>` : ''}
    `;
  }
  async _fetch() {
    this.loading = true;
    this.error = '';
    this.scramble = '';
    try {
      const resp = await fetch(`/api/scramble?length=${this.length}`);
      if (!resp.ok) throw new Error(`HTTP ${resp.status}`);
      const data = await resp.json();
      this.scramble = data.scramble;
    } catch (e) {
      this.error = 'Failed: ' + e.message;
    } finally {
      this.loading = false;
    }
  }
}
customElements.define('scramble-app', ScrambleApp);

class MemoApp extends LitElement {
  static styles = css`
    .spinner {border:4px solid #f3f3f3;border-top:4px solid #3498db;border-radius:50%;width:20px;height:20px;animation:spin 1s linear infinite;display:inline-block;vertical-align:middle;}
    @keyframes spin {to{transform:rotate(360deg);}}
    .error {color:#c00; margin-top:0.5rem;}
    .output {background:#eee;padding:1rem;white-space:pre-wrap;}
    label {display:block;margin-bottom:0.5rem;}
    button {margin-top:0.5rem;}
  `;
  static properties = {
    method: {type: String},
    loading: {type: Boolean},
    result: {type: Object},
    error: {type: String},
  };
  constructor() {
    super();
    this.method = 'old';
    this.loading = false;
    this.result = null;
    this.error = '';
  }
  render() {
    return html`
      <h2>Memo Session</h2>
      <label>Method:
        <select .value=${this.method} @change=${e=>this.method = e.target.value}>
          <option value="old">Old Pochmann / M2</option>
          <option value="three">Three‑style</option>
        </select>
      </label>
      <button ?disabled=${this.loading} @click=${this._start}>Start</button>
      ${this.loading ? html`<span class="spinner"></span>` : ''}
      ${this.error ? html`<div class="error">${this.error}</div>` : ''}
      ${this.result ? html`<pre class="output">${JSON.stringify(this.result, null, 2)}</pre>` : ''}
    `;
  }
  async _start() {
    this.loading = true;
    this.error = '';
    this.result = null;
    try {
      const resp = await fetch('/api/memo', {
        method: 'POST',
        headers: {'Content-Type': 'application/json'},
        body: JSON.stringify({method: this.method})
      });
      if (!resp.ok) throw new Error(`HTTP ${resp.status}`);
      const data = await resp.json();
      this.result = data;
    } catch (e) {
      this.error = 'Failed: ' + e.message;
    } finally {
      this.loading = false;
    }
  }
}
customElements.define('memo-app', MemoApp);
