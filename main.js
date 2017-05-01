"use strict";
(function () {
    var a11yOutputEl = el_with_id('a11y-output');
    var outputEl = el_with_id('output');
    var promptEl = el_with_id('prompt');
    var inputEl = el_with_id('input');
    var formEl = el_with_id('form');
    if (!(inputEl instanceof HTMLInputElement))
        throw new Error("Expected inputEl to be an <input>");
    if (!(formEl instanceof HTMLFormElement))
        throw new Error("Expected formEl to be a <form>");
    var _currentInput = null;
    var _currentPrompt = promptEl.textContent;
    // This is a hack that allows us to make it seem as though the program
    // is "sleeping" when it wants to: because delays are only used for dramatic
    // effect, we'll use a promise to buffer I/O and prevent the user from
    // seeing any output (or sending any input) until time has passed. In
    // reality, however, the program is running without actually sleeping,
    // unlike its synchronous command-line counterpart.
    var _currentPromise = Promise.resolve();
    var _isSleeping = false;
    function el_with_id(id) {
        var el = document.getElementById(id);
        if (el === null)
            throw new Error("Element with id \"" + id + "\" not found!");
        return el;
    }
    function set_input(val) {
        _currentPromise.then(function () {
            _currentInput = typeof (val) === 'string' ? val.trim() : val;
        });
    }
    function scroll_output() {
        // Different browsers use different elements for scrolling. :(
        [document.documentElement, document.body].forEach(function (el) {
            el.scrollTop = el.scrollHeight;
        });
    }
    window.sleep = function (ms) {
        _currentPromise = _currentPromise.then(function () {
            _isSleeping = true;
            return new Promise(function (resolve) {
                window.setTimeout(function () {
                    _isSleeping = false;
                    resolve();
                }, ms);
            });
        });
    };
    window.clear_screen = function () {
        _currentPromise.then(function () {
            outputEl.textContent = "";
            scroll_output();
        });
    };
    window.set_prompt = function (prompt) {
        _currentPromise.then(function () {
            if (prompt !== _currentPrompt) {
                promptEl.textContent = _currentPrompt = prompt;
                a11yOutputEl.appendChild(document.createTextNode(prompt));
            }
        });
    };
    window.has_input = function () {
        return _currentInput === null ? 0 : 1;
    };
    window.get_input = function () {
        var input = _currentInput;
        _currentInput = null;
        if (input === null) {
            throw new Error("Assertion failure: get_input() should only " +
                "be called when has_input() returns 1.");
        }
        return input;
    };
    window.terminate_program = function () {
        _currentPromise.then(function () {
            window.set_prompt("");
            inputEl.value = "";
            inputEl.disabled = true;
        });
    };
    window.Module = {
        print: function (msg) {
            _currentPromise.then(function () {
                var textNode = document.createTextNode(msg + '\n');
                outputEl.appendChild(textNode);
                a11yOutputEl.appendChild(textNode.cloneNode());
                scroll_output();
            });
        }
    };
    formEl.addEventListener('submit', function (e) {
        e.preventDefault();
        if (_isSleeping)
            return;
        var el = document.createElement('div');
        el.setAttribute('class', 'prompt-response');
        el.textContent = "" + _currentPrompt + inputEl.value;
        outputEl.appendChild(el);
        scroll_output();
        set_input(inputEl.value);
        inputEl.value = "";
    });
    window.addEventListener('DOMContentLoaded', function () {
        var script = document.createElement('script');
        var scriptName = 'werewolves-and-wanderer';
        var suffix = '.js';
        if (!('WebAssembly' in window)) {
            suffix = '.asm.js';
        }
        script.setAttribute('src', scriptName + suffix);
        document.body.appendChild(script);
    });
})();
