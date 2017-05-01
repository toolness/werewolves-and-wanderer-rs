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
    function init_ugh_mobile_support() {
        // If we ever detect that the inner height of the window is
        // this ridiculously tiny, we're probably in a mobile browser
        // with a virtual keyboard that takes up most of the screen.
        var UGH_MOBILE_HEIGHT = 300;
        var ughMobileInterval = setInterval(function () {
            if (window.innerHeight <= UGH_MOBILE_HEIGHT) {
                clearInterval(ughMobileInterval);
                document.documentElement.classList.add('ugh-mobile');
            }
        }, 1000);
    }
    function scroll_output() {
        // We want the very bottom of our input field (i.e., the
        // "virtual console cursor") to be at the bottom of the user's
        // viewport. This is particularly hard to do on iOS Safari, where
        // window.innerHeight doesn't account for the user's keyboard,
        // but at least this algorithm makes things slightly less horrible.
        var PADDING = 8;
        var rect = inputEl.getBoundingClientRect();
        var scrollY = typeof (window.scrollY) === 'number'
            ? window.scrollY
            : window.pageYOffset;
        var bottom = scrollY + rect.bottom + PADDING;
        var scrollTop = Math.max(bottom - window.innerHeight, 0);
        // Different browsers use different elements for scrolling. :(
        [document.documentElement, document.body].forEach(function (el) {
            el.scrollTop = scrollTop;
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
                scroll_output();
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
        init_ugh_mobile_support();
    });
})();