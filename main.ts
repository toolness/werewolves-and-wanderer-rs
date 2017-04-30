"use strict";

interface Window {
  // This is called from Rust when the program wants to sleep.
  sleep: (ms: number) => void;

  // Called from Rust code when it wants to clear the screen.
  clear_screen: () => void;

  // Called from Rust code when it wants to prompt the user for input.
  set_prompt: (prompt: string) => void;

  // Called from Rust code when it wants to know if the user has
  // typed a line of input and pressed return. We would return a boolean,
  // but it's easier to just return an int (0 or 1) via emscripten.
  has_input: () => number;

  // Called from Rust code when it wants to retrieve the user's line
  // of input. It should only be called when has_input() returns 1.
  // This is also a destructive operation, in that has_input() will
  // return 0 after it's called.
  get_input: () => string;

  // Called from Rust code when the program has terminated.
  terminate_program: () => void;

  // This is part of Emscripten's API. For more details, see:
  // http://kripken.github.io/emscripten-site/docs/api_reference/module.html
  Module: {
    // This is ultimately called by any Rust code that writes to
    // stdout.
    print: (msg: string) => void;
  };
}

(() => {
  const outputEl = el_with_id('output');
  const promptEl = el_with_id('prompt');
  const inputEl = el_with_id('input');
  const formEl = el_with_id('form');

  if (!(inputEl instanceof HTMLInputElement))
    throw new Error("Expected inputEl to be an <input>");

  if (!(formEl instanceof HTMLFormElement))
    throw new Error("Expected formEl to be a <form>");

  let _currentInput: string | null = null;
  let _currentPrompt = promptEl.textContent;

  // This is a hack that allows us to make it seem as though the program
  // is "sleeping" when it wants to: because delays are only used for dramatic
  // effect, we'll use a promise to buffer I/O and prevent the user from
  // seeing any output (or sending any input) until time has passed. In
  // reality, however, the program is running without actually sleeping,
  // unlike its synchronous command-line counterpart.
  let _currentPromise = Promise.resolve();
  let _isSleeping = false;

  function el_with_id(id: string): HTMLElement {
    const el = document.getElementById(id);
    if (el === null)
      throw new Error(`Element with id "${id}" not found!`);
    return el;
  }

  function set_input(val: string | null) {
    _currentPromise.then(() => {
      _currentInput = typeof(val) === 'string' ? val.trim() : val;
    });
  }

  function scroll_output() {
    // Different browsers use different elements for scrolling. :(
    [document.documentElement, document.body].forEach(el => {
      el.scrollTop = el.scrollHeight;
    });
  }

  window.sleep = (ms: number) => {
    _currentPromise = _currentPromise.then(() => {
      _isSleeping = true;
      return new Promise<void>(resolve => {
        window.setTimeout(() => {
          _isSleeping = false;
          resolve();
        }, ms);
      });
    });
  };

  window.clear_screen = () => {
    _currentPromise.then(() => {
      outputEl.textContent = "";
      scroll_output();
    });
  };

  window.set_prompt = prompt => {
    _currentPromise.then(() => {
      if (prompt !== _currentPrompt) {
        promptEl.textContent = _currentPrompt = prompt;
      }
    });
  };

  window.has_input = () => {
    return _currentInput === null ? 0 : 1;
  };

  window.get_input = () => {
    const input = _currentInput;
    _currentInput = null;
    if (input === null) {
      throw new Error("Assertion failure: get_input() should only " +
                      "be called when has_input() returns 1.");
    }
    return input;
  };

  window.terminate_program = () => {
    _currentPromise.then(() => {
      window.set_prompt("");
      inputEl.value = "";
      inputEl.disabled = true;
    });
  };

  window.Module = {
    print(msg: string) {
      _currentPromise.then(() => {
        const textNode = document.createTextNode(msg + '\n');
        outputEl.appendChild(textNode);
        scroll_output();
      });
    }
  };

  formEl.addEventListener('submit', e => {
    e.preventDefault();

    if (_isSleeping) return;

    const el = document.createElement('div');

    el.setAttribute('class', 'prompt-response');
    el.textContent = `${_currentPrompt}${inputEl.value}`;
    outputEl.appendChild(el);
    scroll_output();

    window.set_prompt("");
    set_input(inputEl.value);
    inputEl.value = "";
  });

  window.addEventListener('DOMContentLoaded', () => {
    const script = document.createElement('script');
    const scriptName = 'werewolves-and-wanderer';
    let suffix = '.js';

    if (!('WebAssembly' in window)) {
      suffix = '.asm.js';
    }

    script.setAttribute('src', scriptName + suffix);
    document.body.appendChild(script);
  });
})();
