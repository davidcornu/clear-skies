var et = Object.defineProperty;
var tt = (n, e, t) => e in n ? et(n, e, { enumerable: !0, configurable: !0, writable: !0, value: t }) : n[e] = t;
var U = (n, e, t) => (tt(n, typeof e != "symbol" ? e + "" : e, t), t), Oe = (n, e, t) => {
  if (!e.has(n))
    throw TypeError("Cannot " + t);
};
var w = (n, e, t) => (Oe(n, e, "read from private field"), t ? t.call(n) : e.get(n)), W = (n, e, t) => {
  if (e.has(n))
    throw TypeError("Cannot add the same private member more than once");
  e instanceof WeakSet ? e.add(n) : e.set(n, t);
}, H = (n, e, t, r) => (Oe(n, e, "write to private field"), r ? r.call(n, t) : e.set(n, t), t);
function I() {
}
function nt(n) {
  return !!n && (typeof n == "object" || typeof n == "function") && typeof /** @type {any} */
  n.then == "function";
}
function Qe(n) {
  return n();
}
function De() {
  return /* @__PURE__ */ Object.create(null);
}
function ue(n) {
  n.forEach(Qe);
}
function Ye(n) {
  return typeof n == "function";
}
function Se(n, e) {
  return n != n ? e == e : n !== e || n && typeof n == "object" || typeof n == "function";
}
function rt(n) {
  return Object.keys(n).length === 0;
}
function b(n, e) {
  n.appendChild(e);
}
function S(n, e, t) {
  n.insertBefore(e, t || null);
}
function $(n) {
  n.parentNode && n.parentNode.removeChild(n);
}
function pe(n, e) {
  for (let t = 0; t < n.length; t += 1)
    n[t] && n[t].d(e);
}
function y(n) {
  return document.createElement(n);
}
function T(n) {
  return document.createTextNode(n);
}
function L() {
  return T(" ");
}
function _e() {
  return T("");
}
function E(n, e, t) {
  t == null ? n.removeAttribute(e) : n.getAttribute(e) !== t && n.setAttribute(e, t);
}
function st(n) {
  return Array.from(n.childNodes);
}
function V(n, e) {
  e = "" + e, n.data !== e && (n.data = /** @type {string} */
  e);
}
function N(n, e, t, r) {
  t == null ? n.style.removeProperty(e) : n.style.setProperty(e, t, r ? "important" : "");
}
let oe;
function F(n) {
  oe = n;
}
function Re() {
  if (!oe)
    throw new Error("Function called outside component initialization");
  return oe;
}
function ot(n) {
  Re().$$.on_mount.push(n);
}
function it(n) {
  Re().$$.on_destroy.push(n);
}
const ee = [], qe = [];
let te = [];
const Ie = [], lt = /* @__PURE__ */ Promise.resolve();
let we = !1;
function ut() {
  we || (we = !0, lt.then(Ee));
}
function $e(n) {
  te.push(n);
}
const ye = /* @__PURE__ */ new Set();
let X = 0;
function Ee() {
  if (X !== 0)
    return;
  const n = oe;
  do {
    try {
      for (; X < ee.length; ) {
        const e = ee[X];
        X++, F(e), at(e.$$);
      }
    } catch (e) {
      throw ee.length = 0, X = 0, e;
    }
    for (F(null), ee.length = 0, X = 0; qe.length; )
      qe.pop()();
    for (let e = 0; e < te.length; e += 1) {
      const t = te[e];
      ye.has(t) || (ye.add(t), t());
    }
    te.length = 0;
  } while (ee.length);
  for (; Ie.length; )
    Ie.pop()();
  we = !1, ye.clear(), F(n);
}
function at(n) {
  if (n.fragment !== null) {
    n.update(), ue(n.before_update);
    const e = n.dirty;
    n.dirty = [-1], n.fragment && n.fragment.p(n.ctx, e), n.after_update.forEach($e);
  }
}
function ct(n) {
  const e = [], t = [];
  te.forEach((r) => n.indexOf(r) === -1 ? e.push(r) : t.push(r)), t.forEach((r) => r()), te = e;
}
const he = /* @__PURE__ */ new Set();
let K;
function ae() {
  K = {
    r: 0,
    c: [],
    p: K
    // parent group
  };
}
function ce() {
  K.r || ue(K.c), K = K.p;
}
function C(n, e) {
  n && n.i && (he.delete(n), n.i(e));
}
function q(n, e, t, r) {
  if (n && n.o) {
    if (he.has(n))
      return;
    he.add(n), K.c.push(() => {
      he.delete(n), r && (t && n.d(1), r());
    }), n.o(e);
  } else
    r && r();
}
function Ne(n, e) {
  const t = e.token = {};
  function r(o, s, i, u) {
    if (e.token !== t)
      return;
    e.resolved = u;
    let l = e.ctx;
    i !== void 0 && (l = l.slice(), l[i] = u);
    const a = o && (e.current = o)(l);
    let d = !1;
    e.block && (e.blocks ? e.blocks.forEach((f, h) => {
      h !== s && f && (ae(), q(f, 1, 1, () => {
        e.blocks[h] === f && (e.blocks[h] = null);
      }), ce());
    }) : e.block.d(1), a.c(), C(a, 1), a.m(e.mount(), e.anchor), d = !0), e.block = a, e.blocks && (e.blocks[s] = a), d && Ee();
  }
  if (nt(n)) {
    const o = Re();
    if (n.then(
      (s) => {
        F(o), r(e.then, 1, e.value, s), F(null);
      },
      (s) => {
        if (F(o), r(e.catch, 2, e.error, s), F(null), !e.hasCatch)
          throw s;
      }
    ), e.current !== e.pending)
      return r(e.pending, 0), !0;
  } else {
    if (e.current !== e.then)
      return r(e.then, 1, e.value, n), !0;
    e.resolved = /** @type {T} */
    n;
  }
}
function dt(n, e, t) {
  const r = e.slice(), { resolved: o } = n;
  n.current === n.then && (r[n.value] = o), n.current === n.catch && (r[n.error] = o), n.block.p(r, t);
}
function G(n) {
  return (n == null ? void 0 : n.length) !== void 0 ? n : Array.from(n);
}
function ie(n) {
  n && n.c();
}
function re(n, e, t) {
  const { fragment: r, after_update: o } = n.$$;
  r && r.m(e, t), $e(() => {
    const s = n.$$.on_mount.map(Qe).filter(Ye);
    n.$$.on_destroy ? n.$$.on_destroy.push(...s) : ue(s), n.$$.on_mount = [];
  }), o.forEach($e);
}
function se(n, e) {
  const t = n.$$;
  t.fragment !== null && (ct(t.after_update), ue(t.on_destroy), t.fragment && t.fragment.d(e), t.on_destroy = t.fragment = null, t.ctx = []);
}
function ft(n, e) {
  n.$$.dirty[0] === -1 && (ee.push(n), ut(), n.$$.dirty.fill(0)), n.$$.dirty[e / 31 | 0] |= 1 << e % 31;
}
function ke(n, e, t, r, o, s, i = null, u = [-1]) {
  const l = oe;
  F(n);
  const a = n.$$ = {
    fragment: null,
    ctx: [],
    // state
    props: s,
    update: I,
    not_equal: o,
    bound: De(),
    // lifecycle
    on_mount: [],
    on_destroy: [],
    on_disconnect: [],
    before_update: [],
    after_update: [],
    context: new Map(e.context || (l ? l.$$.context : [])),
    // everything else
    callbacks: De(),
    dirty: u,
    skip_bound: !1,
    root: e.target || l.$$.root
  };
  i && i(a.root);
  let d = !1;
  if (a.ctx = t ? t(n, e.props || {}, (f, h, ...m) => {
    const k = m.length ? m[0] : h;
    return a.ctx && o(a.ctx[f], a.ctx[f] = k) && (!a.skip_bound && a.bound[f] && a.bound[f](k), d && ft(n, f)), h;
  }) : [], a.update(), d = !0, ue(a.before_update), a.fragment = r ? r(a.ctx) : !1, e.target) {
    if (e.hydrate) {
      const f = st(e.target);
      a.fragment && a.fragment.l(f), f.forEach($);
    } else
      a.fragment && a.fragment.c();
    e.intro && C(n.$$.fragment), re(n, e.target, e.anchor), Ee();
  }
  F(l);
}
class Le {
  constructor() {
    /**
     * ### PRIVATE API
     *
     * Do not use, may change at any time
     *
     * @type {any}
     */
    U(this, "$$");
    /**
     * ### PRIVATE API
     *
     * Do not use, may change at any time
     *
     * @type {any}
     */
    U(this, "$$set");
  }
  /** @returns {void} */
  $destroy() {
    se(this, 1), this.$destroy = I;
  }
  /**
   * @template {Extract<keyof Events, string>} K
   * @param {K} type
   * @param {((e: Events[K]) => void) | null | undefined} callback
   * @returns {() => void}
   */
  $on(e, t) {
    if (!Ye(t))
      return I;
    const r = this.$$.callbacks[e] || (this.$$.callbacks[e] = []);
    return r.push(t), () => {
      const o = r.indexOf(t);
      o !== -1 && r.splice(o, 1);
    };
  }
  /**
   * @param {Partial<Props>} props
   * @returns {void}
   */
  $set(e) {
    this.$$set && !rt(e) && (this.$$.skip_bound = !0, this.$$set(e), this.$$.skip_bound = !1);
  }
}
const ht = "4";
typeof window < "u" && (window.__svelte || (window.__svelte = { v: /* @__PURE__ */ new Set() })).v.add(ht);
function Ae(n, e) {
  if (!(n instanceof e))
    throw new TypeError("Cannot call a class as a function");
}
function Ue(n, e) {
  for (var t = 0; t < e.length; t++) {
    var r = e[t];
    r.enumerable = r.enumerable || !1, r.configurable = !0, "value" in r && (r.writable = !0), Object.defineProperty(n, r.key, r);
  }
}
function pt(n, e, t) {
  return e && Ue(n.prototype, e), t && Ue(n, t), n;
}
function v(n, e, t) {
  return e in n ? Object.defineProperty(n, e, {
    value: t,
    enumerable: !0,
    configurable: !0,
    writable: !0
  }) : n[e] = t, n;
}
var mt = function(e, t) {
  return e.matches ? e.matches(t) : e.msMatchesSelector ? e.msMatchesSelector(t) : e.webkitMatchesSelector ? e.webkitMatchesSelector(t) : null;
}, _t = function(e, t) {
  for (var r = e; r && r.nodeType === 1; ) {
    if (mt(r, t))
      return r;
    r = r.parentNode;
  }
  return null;
}, bt = function(e, t) {
  return e.closest ? e.closest(t) : _t(e, t);
}, vt = function(e) {
  return !!(e && typeof e.then == "function");
}, yt = function n() {
  var e = this, t = arguments.length > 0 && arguments[0] !== void 0 ? arguments[0] : {}, r = t.search, o = t.autoSelect, s = o === void 0 ? !1 : o, i = t.setValue, u = i === void 0 ? function() {
  } : i, l = t.setAttribute, a = l === void 0 ? function() {
  } : l, d = t.onUpdate, f = d === void 0 ? function() {
  } : d, h = t.onSubmit, m = h === void 0 ? function() {
  } : h, k = t.onShow, c = k === void 0 ? function() {
  } : k, _ = t.autocorrect, p = _ === void 0 ? !1 : _, O = t.onHide, Q = O === void 0 ? function() {
  } : O, J = t.onLoading, be = J === void 0 ? function() {
  } : J, Y = t.onLoaded, A = Y === void 0 ? function() {
  } : Y, P = t.submitOnEnter, ve = P === void 0 ? !1 : P;
  Ae(this, n), v(this, "value", ""), v(this, "searchCounter", 0), v(this, "results", []), v(this, "selectedIndex", -1), v(this, "selectedResult", null), v(this, "destroy", function() {
    e.search = null, e.setValue = null, e.setAttribute = null, e.onUpdate = null, e.onSubmit = null, e.autocorrect = null, e.onShow = null, e.onHide = null, e.onLoading = null, e.onLoaded = null;
  }), v(this, "handleInput", function(g) {
    var R = g.target.value;
    e.updateResults(R), e.value = R;
  }), v(this, "handleKeyDown", function(g) {
    var R = g.key;
    switch (R) {
      case "Up":
      case "Down":
      case "ArrowUp":
      case "ArrowDown": {
        var D = R === "ArrowUp" || R === "Up" ? e.selectedIndex - 1 : e.selectedIndex + 1;
        g.preventDefault(), e.handleArrows(D);
        break;
      }
      case "Tab": {
        e.selectResult();
        break;
      }
      case "Enter": {
        var M = g.target.getAttribute("aria-activedescendant").length > 0;
        e.selectedResult = e.results[e.selectedIndex] || e.selectedResult, e.selectResult(), e.submitOnEnter ? e.selectedResult && e.onSubmit(e.selectedResult) : M ? g.preventDefault() : (e.selectedResult && e.onSubmit(e.selectedResult), e.selectedResult = null);
        break;
      }
      case "Esc":
      case "Escape": {
        e.hideResults(), e.setValue();
        break;
      }
      default:
        return;
    }
  }), v(this, "handleFocus", function(g) {
    var R = g.target.value;
    e.updateResults(R), e.value = R;
  }), v(this, "handleBlur", function() {
    e.hideResults();
  }), v(this, "handleResultMouseDown", function(g) {
    g.preventDefault();
  }), v(this, "handleResultClick", function(g) {
    var R = g.target, D = bt(R, "[data-result-index]");
    if (D) {
      e.selectedIndex = parseInt(D.dataset.resultIndex, 10);
      var M = e.results[e.selectedIndex];
      e.selectResult(), e.onSubmit(M);
    }
  }), v(this, "handleArrows", function(g) {
    var R = e.results.length;
    e.selectedIndex = (g % R + R) % R, e.onUpdate(e.results, e.selectedIndex);
  }), v(this, "selectResult", function() {
    var g = e.results[e.selectedIndex];
    g && e.setValue(g), e.hideResults();
  }), v(this, "updateResults", function(g) {
    var R = ++e.searchCounter;
    e.onLoading(), e.search(g).then(function(D) {
      if (R === e.searchCounter) {
        if (e.results = D, e.onLoaded(), e.results.length === 0) {
          e.hideResults();
          return;
        }
        e.selectedIndex = e.autoSelect ? 0 : -1, e.onUpdate(e.results, e.selectedIndex), e.showResults();
      }
    });
  }), v(this, "showResults", function() {
    e.setAttribute("aria-expanded", !0), e.onShow();
  }), v(this, "hideResults", function() {
    e.selectedIndex = -1, e.results = [], e.setAttribute("aria-expanded", !1), e.setAttribute("aria-activedescendant", ""), e.onUpdate(e.results, e.selectedIndex), e.onHide();
  }), v(this, "checkSelectedResultVisible", function(g) {
    var R = g.querySelector('[data-result-index="'.concat(e.selectedIndex, '"]'));
    if (R) {
      var D = g.getBoundingClientRect(), M = R.getBoundingClientRect();
      M.top < D.top ? g.scrollTop -= D.top - M.top : M.bottom > D.bottom && (g.scrollTop += M.bottom - D.bottom);
    }
  }), this.search = vt(r) ? r : function(g) {
    return Promise.resolve(r(g));
  }, this.autoSelect = s, this.setValue = u, this.setAttribute = a, this.onUpdate = f, this.onSubmit = m, this.autocorrect = p, this.onShow = c, this.onHide = Q, this.onLoading = be, this.onLoaded = A, this.submitOnEnter = ve;
}, gt = 0, wt = function() {
  var e = arguments.length > 0 && arguments[0] !== void 0 ? arguments[0] : "";
  return "".concat(e).concat(++gt);
}, $t = function(e, t) {
  var r = e.getBoundingClientRect(), o = t.getBoundingClientRect(), s = (
    /* 1 */
    r.bottom + o.height > window.innerHeight && /* 2 */
    window.innerHeight - r.bottom < r.top && /* 3 */
    window.pageYOffset + r.top - o.height > 0
  );
  return s ? "above" : "below";
}, St = function(e, t, r) {
  var o;
  return function() {
    var i = this, u = arguments, l = function() {
      o = null, r || e.apply(i, u);
    }, a = r && !o;
    clearTimeout(o), o = setTimeout(l, t), a && e.apply(i, u);
  };
}, Rt = function(e) {
  if (e != null && e.length) {
    var t = e.startsWith("#");
    return {
      attribute: t ? "aria-labelledby" : "aria-label",
      content: t ? e.substring(1) : e
    };
  }
}, Et = /* @__PURE__ */ function() {
  function n(e, t, r) {
    Ae(this, n), this.id = "".concat(r, "-result-").concat(e), this.class = "".concat(r, "-result"), this["data-result-index"] = e, this.role = "option", e === t && (this["aria-selected"] = "true");
  }
  return pt(n, [{
    key: "toString",
    value: function() {
      var t = this;
      return Object.keys(this).reduce(function(r, o) {
        return "".concat(r, " ").concat(o, '="').concat(t[o], '"');
      }, "");
    }
  }]), n;
}(), kt = function n(e) {
  var t = this, r = arguments.length > 1 && arguments[1] !== void 0 ? arguments[1] : {}, o = r.search, s = r.onSubmit, i = s === void 0 ? function() {
  } : s, u = r.onUpdate, l = u === void 0 ? function() {
  } : u, a = r.baseClass, d = a === void 0 ? "autocomplete" : a, f = r.autocorrect, h = f === void 0 ? !1 : f, m = r.autoSelect, k = r.getResultValue, c = k === void 0 ? function(A) {
    return A;
  } : k, _ = r.renderResult, p = r.debounceTime, O = p === void 0 ? 0 : p, Q = r.resultListLabel, J = r.submitOnEnter, be = J === void 0 ? !1 : J;
  Ae(this, n), v(this, "expanded", !1), v(this, "loading", !1), v(this, "position", {}), v(this, "resetPosition", !0), v(this, "initialize", function() {
    t.root.style.position = "relative", t.input.setAttribute("role", "combobox"), t.input.setAttribute("autocomplete", "off"), t.input.setAttribute("autocapitalize", "off"), t.autocorrect && t.input.setAttribute("autocorrect", "on"), t.input.setAttribute("spellcheck", "false"), t.input.setAttribute("aria-autocomplete", "list"), t.input.setAttribute("aria-haspopup", "listbox"), t.input.setAttribute("aria-expanded", "false"), t.resultList.setAttribute("role", "listbox");
    var A = Rt(t.resultListLabel);
    A && t.resultList.setAttribute(A.attribute, A.content), t.resultList.style.position = "absolute", t.resultList.style.zIndex = "1", t.resultList.style.width = "100%", t.resultList.style.boxSizing = "border-box", t.resultList.id || (t.resultList.id = wt("".concat(t.baseClass, "-result-list-"))), t.input.setAttribute("aria-owns", t.resultList.id), document.body.addEventListener("click", t.handleDocumentClick), t.input.addEventListener("input", t.core.handleInput), t.input.addEventListener("keydown", t.core.handleKeyDown), t.input.addEventListener("focus", t.core.handleFocus), t.input.addEventListener("blur", t.core.handleBlur), t.resultList.addEventListener("mousedown", t.core.handleResultMouseDown), t.resultList.addEventListener("click", t.core.handleResultClick), t.updateStyle();
  }), v(this, "destroy", function() {
    document.body.removeEventListener("click", t.handleDocumentClick), t.input.removeEventListener("input", t.core.handleInput), t.input.removeEventListener("keydown", t.core.handleKeyDown), t.input.removeEventListener("focus", t.core.handleFocus), t.input.removeEventListener("blur", t.core.handleBlur), t.resultList.removeEventListener("mousedown", t.core.handleResultMouseDown), t.resultList.removeEventListener("click", t.core.handleResultClick), t.root = null, t.input = null, t.resultList = null, t.getResultValue = null, t.onUpdate = null, t.renderResult = null, t.core.destroy(), t.core = null;
  }), v(this, "setAttribute", function(A, P) {
    t.input.setAttribute(A, P);
  }), v(this, "setValue", function(A) {
    t.input.value = A ? t.getResultValue(A) : "";
  }), v(this, "renderResult", function(A, P) {
    return "<li ".concat(P, ">").concat(t.getResultValue(A), "</li>");
  }), v(this, "handleUpdate", function(A, P) {
    t.resultList.innerHTML = "", A.forEach(function(ve, g) {
      var R = new Et(g, P, t.baseClass), D = t.renderResult(ve, R);
      typeof D == "string" ? t.resultList.insertAdjacentHTML("beforeend", D) : t.resultList.insertAdjacentElement("beforeend", D);
    }), t.input.setAttribute("aria-activedescendant", P > -1 ? "".concat(t.baseClass, "-result-").concat(P) : ""), t.resetPosition && (t.resetPosition = !1, t.position = $t(t.input, t.resultList), t.updateStyle()), t.core.checkSelectedResultVisible(t.resultList), t.onUpdate(A, P);
  }), v(this, "handleShow", function() {
    t.expanded = !0, t.updateStyle();
  }), v(this, "handleHide", function() {
    t.expanded = !1, t.resetPosition = !0, t.updateStyle();
  }), v(this, "handleLoading", function() {
    t.loading = !0, t.updateStyle();
  }), v(this, "handleLoaded", function() {
    t.loading = !1, t.updateStyle();
  }), v(this, "handleDocumentClick", function(A) {
    t.root.contains(A.target) || t.core.hideResults();
  }), v(this, "updateStyle", function() {
    t.root.dataset.expanded = t.expanded, t.root.dataset.loading = t.loading, t.root.dataset.position = t.position, t.resultList.style.visibility = t.expanded ? "visible" : "hidden", t.resultList.style.pointerEvents = t.expanded ? "auto" : "none", t.position === "below" ? (t.resultList.style.bottom = null, t.resultList.style.top = "100%") : (t.resultList.style.top = null, t.resultList.style.bottom = "100%");
  }), this.root = typeof e == "string" ? document.querySelector(e) : e, this.input = this.root.querySelector("input"), this.resultList = this.root.querySelector("ul"), this.baseClass = d, this.autocorrect = h, this.getResultValue = c, this.onUpdate = l, typeof _ == "function" && (this.renderResult = _), this.resultListLabel = Q, this.submitOnEnter = be;
  var Y = new yt({
    search: o,
    autoSelect: m,
    setValue: this.setValue,
    setAttribute: this.setAttribute,
    onUpdate: this.handleUpdate,
    autocorrect: this.autocorrect,
    onSubmit: i,
    onShow: this.handleShow,
    onHide: this.handleHide,
    onLoading: this.handleLoading,
    onLoaded: this.handleLoaded,
    submitOnEnter: this.submitOnEnter
  });
  O > 0 && (Y.handleInput = St(Y.handleInput, O)), this.core = Y, this.initialize();
};
class Lt {
  constructor(e) {
    this.config = e;
  }
}
class Pe extends Error {
  constructor(t, r, o) {
    super(o);
    U(this, "url");
    U(this, "status");
    U(this, "statusText");
    U(this, "body");
    U(this, "request");
    this.name = "ApiError", this.url = r.url, this.status = r.status, this.statusText = r.statusText, this.body = r.body, this.request = t;
  }
}
class At extends Error {
  constructor(e) {
    super(e), this.name = "CancelError";
  }
  get isCancelled() {
    return !0;
  }
}
var B, x, j, z, Z, le, ne;
class Ct {
  constructor(e) {
    W(this, B, void 0);
    W(this, x, void 0);
    W(this, j, void 0);
    W(this, z, void 0);
    W(this, Z, void 0);
    W(this, le, void 0);
    W(this, ne, void 0);
    H(this, B, !1), H(this, x, !1), H(this, j, !1), H(this, z, []), H(this, Z, new Promise((t, r) => {
      H(this, le, t), H(this, ne, r);
      const o = (u) => {
        var l;
        w(this, B) || w(this, x) || w(this, j) || (H(this, B, !0), (l = w(this, le)) == null || l.call(this, u));
      }, s = (u) => {
        var l;
        w(this, B) || w(this, x) || w(this, j) || (H(this, x, !0), (l = w(this, ne)) == null || l.call(this, u));
      }, i = (u) => {
        w(this, B) || w(this, x) || w(this, j) || w(this, z).push(u);
      };
      return Object.defineProperty(i, "isResolved", {
        get: () => w(this, B)
      }), Object.defineProperty(i, "isRejected", {
        get: () => w(this, x)
      }), Object.defineProperty(i, "isCancelled", {
        get: () => w(this, j)
      }), e(o, s, i);
    }));
  }
  get [Symbol.toStringTag]() {
    return "Cancellable Promise";
  }
  then(e, t) {
    return w(this, Z).then(e, t);
  }
  catch(e) {
    return w(this, Z).catch(e);
  }
  finally(e) {
    return w(this, Z).finally(e);
  }
  cancel() {
    var e;
    if (!(w(this, B) || w(this, x) || w(this, j))) {
      if (H(this, j, !0), w(this, z).length)
        try {
          for (const t of w(this, z))
            t();
        } catch (t) {
          console.warn("Cancellation threw an error", t);
          return;
        }
      w(this, z).length = 0, (e = w(this, ne)) == null || e.call(this, new At("Request aborted"));
    }
  }
  get isCancelled() {
    return w(this, j);
  }
}
B = new WeakMap(), x = new WeakMap(), j = new WeakMap(), z = new WeakMap(), Z = new WeakMap(), le = new WeakMap(), ne = new WeakMap();
const Ce = (n) => n != null, de = (n) => typeof n == "string", ge = (n) => de(n) && n !== "", Te = (n) => typeof n == "object" && typeof n.type == "string" && typeof n.stream == "function" && typeof n.arrayBuffer == "function" && typeof n.constructor == "function" && typeof n.constructor.name == "string" && /^(Blob|File)$/.test(n.constructor.name) && /^(Blob|File)$/.test(n[Symbol.toStringTag]), Xe = (n) => n instanceof FormData, Tt = (n) => {
  try {
    return btoa(n);
  } catch {
    return Buffer.from(n).toString("base64");
  }
}, Ot = (n) => {
  const e = [], t = (o, s) => {
    e.push(`${encodeURIComponent(o)}=${encodeURIComponent(String(s))}`);
  }, r = (o, s) => {
    Ce(s) && (Array.isArray(s) ? s.forEach((i) => {
      r(o, i);
    }) : typeof s == "object" ? Object.entries(s).forEach(([i, u]) => {
      r(`${o}[${i}]`, u);
    }) : t(o, s));
  };
  return Object.entries(n).forEach(([o, s]) => {
    r(o, s);
  }), e.length > 0 ? `?${e.join("&")}` : "";
}, Dt = (n, e) => {
  const t = n.ENCODE_PATH || encodeURI, r = e.url.replace("{api-version}", n.VERSION).replace(/{(.*?)}/g, (s, i) => {
    var u;
    return (u = e.path) != null && u.hasOwnProperty(i) ? t(String(e.path[i])) : s;
  }), o = `${n.BASE}${r}`;
  return e.query ? `${o}${Ot(e.query)}` : o;
}, qt = (n) => {
  if (n.formData) {
    const e = new FormData(), t = (r, o) => {
      de(o) || Te(o) ? e.append(r, o) : e.append(r, JSON.stringify(o));
    };
    return Object.entries(n.formData).filter(([r, o]) => Ce(o)).forEach(([r, o]) => {
      Array.isArray(o) ? o.forEach((s) => t(r, s)) : t(r, o);
    }), e;
  }
}, fe = async (n, e) => typeof e == "function" ? e(n) : e, It = async (n, e) => {
  const t = await fe(e, n.TOKEN), r = await fe(e, n.USERNAME), o = await fe(e, n.PASSWORD), s = await fe(e, n.HEADERS), i = Object.entries({
    Accept: "application/json",
    ...s,
    ...e.headers
  }).filter(([u, l]) => Ce(l)).reduce((u, [l, a]) => ({
    ...u,
    [l]: String(a)
  }), {});
  if (ge(t) && (i.Authorization = `Bearer ${t}`), ge(r) && ge(o)) {
    const u = Tt(`${r}:${o}`);
    i.Authorization = `Basic ${u}`;
  }
  return e.body && (e.mediaType ? i["Content-Type"] = e.mediaType : Te(e.body) ? i["Content-Type"] = e.body.type || "application/octet-stream" : de(e.body) ? i["Content-Type"] = "text/plain" : Xe(e.body) || (i["Content-Type"] = "application/json")), new Headers(i);
}, Nt = (n) => {
  var e;
  if (n.body !== void 0)
    return (e = n.mediaType) != null && e.includes("/json") ? JSON.stringify(n.body) : de(n.body) || Te(n.body) || Xe(n.body) ? n.body : JSON.stringify(n.body);
}, Ut = async (n, e, t, r, o, s, i) => {
  const u = new AbortController(), l = {
    headers: s,
    body: r ?? o,
    method: e.method,
    signal: u.signal
  };
  return n.WITH_CREDENTIALS && (l.credentials = n.CREDENTIALS), i(() => u.abort()), await fetch(t, l);
}, Pt = (n, e) => {
  if (e) {
    const t = n.headers.get(e);
    if (de(t))
      return t;
  }
}, Ht = async (n) => {
  if (n.status !== 204)
    try {
      const e = n.headers.get("Content-Type");
      if (e)
        return ["application/json", "application/problem+json"].some((o) => e.toLowerCase().startsWith(o)) ? await n.json() : await n.text();
    } catch (e) {
      console.error(e);
    }
}, jt = (n, e) => {
  const r = {
    400: "Bad Request",
    401: "Unauthorized",
    403: "Forbidden",
    404: "Not Found",
    500: "Internal Server Error",
    502: "Bad Gateway",
    503: "Service Unavailable",
    ...n.errors
  }[e.status];
  if (r)
    throw new Pe(n, e, r);
  if (!e.ok) {
    const o = e.status ?? "unknown", s = e.statusText ?? "unknown", i = (() => {
      try {
        return JSON.stringify(e.body, null, 2);
      } catch {
        return;
      }
    })();
    throw new Pe(
      n,
      e,
      `Generic Error: status: ${o}; status text: ${s}; body: ${i}`
    );
  }
}, Bt = (n, e) => new Ct(async (t, r, o) => {
  try {
    const s = Dt(n, e), i = qt(e), u = Nt(e), l = await It(n, e);
    if (!o.isCancelled) {
      const a = await Ut(n, e, s, u, i, l, o), d = await Ht(a), f = Pt(a, e.responseHeader), h = {
        url: s,
        ok: a.ok,
        status: a.status,
        statusText: a.statusText,
        body: f ?? d
      };
      jt(e, h), t(h.body);
    }
  } catch (s) {
    r(s);
  }
});
class xt extends Lt {
  constructor(e) {
    super(e);
  }
  /**
   * Request method
   * @param options The request options from the service
   * @returns CancelablePromise<T>
   * @throws ApiError
   */
  request(e) {
    return Bt(this.config, e);
  }
}
class Vt {
  constructor(e) {
    this.httpRequest = e;
  }
  /**
   * Returns the OpenAPI v3.0.3 specification for this server.
   * @returns any successful operation
   * @throws ApiError
   */
  openapiSchema() {
    return this.httpRequest.request({
      method: "GET",
      url: "/openapi.json"
    });
  }
}
class Ft {
  constructor(e) {
    this.httpRequest = e;
  }
  /**
   * Listing of all available locations
   * @returns LocationResultsPage successful operation
   * @throws ApiError
   */
  locations({
    limit: e,
    pageToken: t
  }) {
    return this.httpRequest.request({
      method: "GET",
      url: "/locations",
      query: {
        limit: e,
        page_token: t
      }
    });
  }
  /**
   * Fuzzy search all available locations
   * @returns Location successful operation
   * @throws ApiError
   */
  locationsSearch({
    q: e
  }) {
    return this.httpRequest.request({
      method: "GET",
      url: "/locations/search",
      query: {
        q: e
      }
    });
  }
}
class Mt {
  constructor(e) {
    this.httpRequest = e;
  }
  /**
   * @returns WeatherReport successful operation
   * @throws ApiError
   */
  weather({
    provinceOrTerritory: e,
    slug: t
  }) {
    return this.httpRequest.request({
      method: "GET",
      url: "/weather/{province_or_territory}/{slug}",
      path: {
        province_or_territory: e,
        slug: t
      }
    });
  }
}
class Wt {
  constructor(e, t = xt) {
    U(this, "documentation");
    U(this, "locations");
    U(this, "weather");
    U(this, "request");
    this.request = new t({
      BASE: (e == null ? void 0 : e.BASE) ?? "",
      VERSION: (e == null ? void 0 : e.VERSION) ?? "0.1.0",
      WITH_CREDENTIALS: (e == null ? void 0 : e.WITH_CREDENTIALS) ?? !1,
      CREDENTIALS: (e == null ? void 0 : e.CREDENTIALS) ?? "include",
      TOKEN: e == null ? void 0 : e.TOKEN,
      USERNAME: e == null ? void 0 : e.USERNAME,
      PASSWORD: e == null ? void 0 : e.PASSWORD,
      HEADERS: e == null ? void 0 : e.HEADERS,
      ENCODE_PATH: e == null ? void 0 : e.ENCODE_PATH
    }), this.documentation = new Vt(this.request), this.locations = new Ft(this.request), this.weather = new Mt(this.request);
  }
}
function He(n) {
  let e, t, r = (
    /*forecast*/
    n[0].probability_of_precipitation + ""
  ), o, s;
  return {
    c() {
      e = y("div"), t = T("☔️ "), o = T(r), s = T("%"), E(e, "class", "forecast--pop svelte-1l8uyn8");
    },
    m(i, u) {
      S(i, e, u), b(e, t), b(e, o), b(e, s);
    },
    p(i, u) {
      u & /*forecast*/
      1 && r !== (r = /*forecast*/
      i[0].probability_of_precipitation + "") && V(o, r);
    },
    d(i) {
      i && $(e);
    }
  };
}
function zt(n) {
  let e, t, r, o = (
    /*forecast*/
    n[0].temperature.degrees_c + ""
  ), s, i, u, l, a = (
    /*forecast*/
    n[0].condition + ""
  ), d, f, h = (
    /*forecast*/
    n[0].probability_of_precipitation && He(n)
  );
  return {
    c() {
      e = y("div"), t = y("div"), r = y("strong"), s = T(o), i = T("°C"), u = L(), l = y("div"), d = T(a), f = L(), h && h.c(), E(t, "class", "forecast--temperature svelte-1l8uyn8"), E(l, "class", "forecast--condition svelte-1l8uyn8"), E(e, "class", "forecast--cell svelte-1l8uyn8");
    },
    m(m, k) {
      S(m, e, k), b(e, t), b(t, r), b(r, s), b(r, i), b(e, u), b(e, l), b(l, d), b(e, f), h && h.m(e, null);
    },
    p(m, [k]) {
      k & /*forecast*/
      1 && o !== (o = /*forecast*/
      m[0].temperature.degrees_c + "") && V(s, o), k & /*forecast*/
      1 && a !== (a = /*forecast*/
      m[0].condition + "") && V(d, a), /*forecast*/
      m[0].probability_of_precipitation ? h ? h.p(m, k) : (h = He(m), h.c(), h.m(e, null)) : h && (h.d(1), h = null);
    },
    i: I,
    o: I,
    d(m) {
      m && $(e), h && h.d();
    }
  };
}
function Gt(n, e, t) {
  let { forecast: r } = e;
  return n.$$set = (o) => {
    "forecast" in o && t(0, r = o.forecast);
  }, [r];
}
class me extends Le {
  constructor(e) {
    super(), ke(this, e, Gt, zt, Se, { forecast: 0 });
  }
}
function je(n, e, t) {
  const r = n.slice();
  r[12] = e[t], r[15] = t;
  const o = (
    /*idx*/
    r[15] + 2
  );
  return r[13] = o, r;
}
function Be(n, e, t) {
  const r = n.slice();
  r[12] = e[t], r[15] = t;
  const o = (
    /*idx*/
    r[15] + 2
  );
  r[13] = o;
  const s = (
    /*dayHeader*/
    r[2](
      /*day*/
      r[12].date
    )
  );
  return r[16] = s, r;
}
function xe(n, e, t) {
  const r = n.slice();
  return r[18] = e[t][0], r[19] = e[t][1], r;
}
function Ve(n, e, t) {
  const r = n.slice();
  return r[22] = e[t], r;
}
function Fe(n) {
  let e, t, r, o = G(
    /*report*/
    n[0].special_weather_statements
  ), s = [];
  for (let i = 0; i < o.length; i += 1)
    s[i] = Me(Ve(n, o, i));
  return {
    c() {
      e = y("h3"), e.textContent = "Special Weather Statements", t = L();
      for (let i = 0; i < s.length; i += 1)
        s[i].c();
      r = _e();
    },
    m(i, u) {
      S(i, e, u), S(i, t, u);
      for (let l = 0; l < s.length; l += 1)
        s[l] && s[l].m(i, u);
      S(i, r, u);
    },
    p(i, u) {
      if (u & /*report*/
      1) {
        o = G(
          /*report*/
          i[0].special_weather_statements
        );
        let l;
        for (l = 0; l < o.length; l += 1) {
          const a = Ve(i, o, l);
          s[l] ? s[l].p(a, u) : (s[l] = Me(a), s[l].c(), s[l].m(r.parentNode, r));
        }
        for (; l < s.length; l += 1)
          s[l].d(1);
        s.length = o.length;
      }
    },
    d(i) {
      i && ($(e), $(t), $(r)), pe(s, i);
    }
  };
}
function Me(n) {
  let e, t, r = (
    /*sws*/
    n[22].title + ""
  ), o, s, i, u = (
    /*sws*/
    n[22].summary + ""
  ), l, a, d, f = (
    /*sws*/
    n[22].url + ""
  ), h, m, k;
  return {
    c() {
      e = y("div"), t = y("h4"), o = T(r), s = L(), i = y("p"), l = T(u), a = L(), d = y("a"), h = T(f), k = L(), E(d, "href", m = /*sws*/
      n[22].url), E(e, "class", "special-weather-statement svelte-tm4vmm");
    },
    m(c, _) {
      S(c, e, _), b(e, t), b(t, o), b(e, s), b(e, i), b(i, l), b(e, a), b(e, d), b(d, h), b(e, k);
    },
    p(c, _) {
      _ & /*report*/
      1 && r !== (r = /*sws*/
      c[22].title + "") && V(o, r), _ & /*report*/
      1 && u !== (u = /*sws*/
      c[22].summary + "") && V(l, u), _ & /*report*/
      1 && f !== (f = /*sws*/
      c[22].url + "") && V(h, f), _ & /*report*/
      1 && m !== (m = /*sws*/
      c[22].url) && E(d, "href", m);
    },
    d(c) {
      c && $(e);
    }
  };
}
function We(n) {
  let e, t, r, o = G(
    /*ccFields*/
    n[1]
  ), s = [];
  for (let i = 0; i < o.length; i += 1)
    s[i] = ze(xe(n, o, i));
  return {
    c() {
      e = y("h3"), e.textContent = "Current Conditions", t = L(), r = y("div");
      for (let i = 0; i < s.length; i += 1)
        s[i].c();
      E(r, "class", "cards svelte-tm4vmm");
    },
    m(i, u) {
      S(i, e, u), S(i, t, u), S(i, r, u);
      for (let l = 0; l < s.length; l += 1)
        s[l] && s[l].m(r, null);
    },
    p(i, u) {
      if (u & /*ccFields*/
      2) {
        o = G(
          /*ccFields*/
          i[1]
        );
        let l;
        for (l = 0; l < o.length; l += 1) {
          const a = xe(i, o, l);
          s[l] ? s[l].p(a, u) : (s[l] = ze(a), s[l].c(), s[l].m(r, null));
        }
        for (; l < s.length; l += 1)
          s[l].d(1);
        s.length = o.length;
      }
    },
    d(i) {
      i && ($(e), $(t), $(r)), pe(s, i);
    }
  };
}
function ze(n) {
  let e, t, r = (
    /*field*/
    n[18] + ""
  ), o, s, i, u = (
    /*value*/
    n[19] + ""
  ), l, a;
  return {
    c() {
      e = y("div"), t = y("div"), o = T(r), s = L(), i = y("div"), l = T(u), a = L(), E(t, "class", "card--heading svelte-tm4vmm"), E(i, "class", "card--body svelte-tm4vmm"), E(e, "class", "card svelte-tm4vmm");
    },
    m(d, f) {
      S(d, e, f), b(e, t), b(t, o), b(e, s), b(e, i), b(i, l), b(e, a);
    },
    p(d, f) {
      f & /*ccFields*/
      2 && r !== (r = /*field*/
      d[18] + "") && V(o, r), f & /*ccFields*/
      2 && u !== (u = /*value*/
      d[19] + "") && V(l, u);
    },
    d(d) {
      d && $(e);
    }
  };
}
function Ge(n) {
  let e, t, r, o, s, i, u, l, a, d = G(
    /*report*/
    n[0].weather_forecasts
  ), f = [];
  for (let c = 0; c < d.length; c += 1)
    f[c] = Je(Be(n, d, c));
  let h = G(
    /*report*/
    n[0].weather_forecasts
  ), m = [];
  for (let c = 0; c < h.length; c += 1)
    m[c] = Ke(je(n, h, c));
  const k = (c) => q(m[c], 1, 1, () => {
    m[c] = null;
  });
  return {
    c() {
      e = y("h3"), e.textContent = "Forecast", t = L(), r = y("div"), o = y("div"), o.textContent = "Day", s = L(), i = y("div"), i.textContent = "Night", u = L();
      for (let c = 0; c < f.length; c += 1)
        f[c].c();
      l = L();
      for (let c = 0; c < m.length; c += 1)
        m[c].c();
      E(o, "class", "forecast--time-of-day"), N(o, "grid-column", "2"), N(o, "grid-row", "1"), E(i, "class", "forecast--time-of-day"), N(i, "grid-column", "3"), N(i, "grid-row", "1"), E(r, "class", "forecasts card svelte-tm4vmm");
    },
    m(c, _) {
      S(c, e, _), S(c, t, _), S(c, r, _), b(r, o), b(r, s), b(r, i), b(r, u);
      for (let p = 0; p < f.length; p += 1)
        f[p] && f[p].m(r, null);
      b(r, l);
      for (let p = 0; p < m.length; p += 1)
        m[p] && m[p].m(r, null);
      a = !0;
    },
    p(c, _) {
      if (_ & /*report, dayHeader*/
      5) {
        d = G(
          /*report*/
          c[0].weather_forecasts
        );
        let p;
        for (p = 0; p < d.length; p += 1) {
          const O = Be(c, d, p);
          f[p] ? f[p].p(O, _) : (f[p] = Je(O), f[p].c(), f[p].m(r, l));
        }
        for (; p < f.length; p += 1)
          f[p].d(1);
        f.length = d.length;
      }
      if (_ & /*report*/
      1) {
        h = G(
          /*report*/
          c[0].weather_forecasts
        );
        let p;
        for (p = 0; p < h.length; p += 1) {
          const O = je(c, h, p);
          m[p] ? (m[p].p(O, _), C(m[p], 1)) : (m[p] = Ke(O), m[p].c(), C(m[p], 1), m[p].m(r, null));
        }
        for (ae(), p = h.length; p < m.length; p += 1)
          k(p);
        ce();
      }
    },
    i(c) {
      if (!a) {
        for (let _ = 0; _ < h.length; _ += 1)
          C(m[_]);
        a = !0;
      }
    },
    o(c) {
      m = m.filter(Boolean);
      for (let _ = 0; _ < m.length; _ += 1)
        q(m[_]);
      a = !1;
    },
    d(c) {
      c && ($(e), $(t), $(r)), pe(f, c), pe(m, c);
    }
  };
}
function Je(n) {
  let e, t, r, o = (
    /*fields*/
    n[16].dayOfWeek + ""
  ), s, i, u, l = (
    /*fields*/
    n[16].shortDate + ""
  ), a, d;
  return {
    c() {
      e = y("div"), t = y("div"), r = y("strong"), s = T(o), i = L(), u = y("div"), a = T(l), E(t, "class", "svelte-tm4vmm"), E(u, "class", "svelte-tm4vmm"), E(e, "class", "forecast--day-of-week svelte-tm4vmm"), E(e, "title", d = /*day*/
      n[12].date), N(
        e,
        "grid-row",
        /*row*/
        n[13]
      ), N(e, "grid-column", "1");
    },
    m(f, h) {
      S(f, e, h), b(e, t), b(t, r), b(r, s), b(e, i), b(e, u), b(u, a);
    },
    p(f, h) {
      h & /*report*/
      1 && o !== (o = /*fields*/
      f[16].dayOfWeek + "") && V(s, o), h & /*report*/
      1 && l !== (l = /*fields*/
      f[16].shortDate + "") && V(a, l), h & /*report*/
      1 && d !== (d = /*day*/
      f[12].date) && E(e, "title", d);
    },
    d(f) {
      f && $(e);
    }
  };
}
function Jt(n) {
  let e, t, r, o;
  return t = new me({
    props: {
      forecast: (
        /*day*/
        n[12].forecast.content
      )
    }
  }), {
    c() {
      e = y("div"), ie(t.$$.fragment), r = L(), N(
        e,
        "grid-row",
        /*row*/
        n[13]
      ), N(e, "grid-column", "3");
    },
    m(s, i) {
      S(s, e, i), re(t, e, null), b(e, r), o = !0;
    },
    p(s, i) {
      const u = {};
      i & /*report*/
      1 && (u.forecast = /*day*/
      s[12].forecast.content), t.$set(u);
    },
    i(s) {
      o || (C(t.$$.fragment, s), o = !0);
    },
    o(s) {
      q(t.$$.fragment, s), o = !1;
    },
    d(s) {
      s && $(e), se(t);
    }
  };
}
function Kt(n) {
  let e, t, r, o;
  return t = new me({
    props: {
      forecast: (
        /*day*/
        n[12].forecast.content
      )
    }
  }), {
    c() {
      e = y("div"), ie(t.$$.fragment), r = L(), N(
        e,
        "grid-row",
        /*row*/
        n[13]
      ), N(e, "grid-column", "2 / span 2");
    },
    m(s, i) {
      S(s, e, i), re(t, e, null), b(e, r), o = !0;
    },
    p(s, i) {
      const u = {};
      i & /*report*/
      1 && (u.forecast = /*day*/
      s[12].forecast.content), t.$set(u);
    },
    i(s) {
      o || (C(t.$$.fragment, s), o = !0);
    },
    o(s) {
      q(t.$$.fragment, s), o = !1;
    },
    d(s) {
      s && $(e), se(t);
    }
  };
}
function Zt(n) {
  let e, t, r, o, s, i, u;
  return t = new me({
    props: {
      forecast: (
        /*day*/
        n[12].forecast.content.day
      )
    }
  }), s = new me({
    props: {
      forecast: (
        /*day*/
        n[12].forecast.content.night
      )
    }
  }), {
    c() {
      e = y("div"), ie(t.$$.fragment), r = L(), o = y("div"), ie(s.$$.fragment), i = L(), N(
        e,
        "grid-row",
        /*row*/
        n[13]
      ), N(e, "grid-column", "2"), N(
        o,
        "grid-row",
        /*row*/
        n[13]
      ), N(o, "grid-column", "3");
    },
    m(l, a) {
      S(l, e, a), re(t, e, null), S(l, r, a), S(l, o, a), re(s, o, null), b(o, i), u = !0;
    },
    p(l, a) {
      const d = {};
      a & /*report*/
      1 && (d.forecast = /*day*/
      l[12].forecast.content.day), t.$set(d);
      const f = {};
      a & /*report*/
      1 && (f.forecast = /*day*/
      l[12].forecast.content.night), s.$set(f);
    },
    i(l) {
      u || (C(t.$$.fragment, l), C(s.$$.fragment, l), u = !0);
    },
    o(l) {
      q(t.$$.fragment, l), q(s.$$.fragment, l), u = !1;
    },
    d(l) {
      l && ($(e), $(r), $(o)), se(t), se(s);
    }
  };
}
function Ke(n) {
  let e, t, r, o;
  const s = [Zt, Kt, Jt], i = [];
  function u(l, a) {
    return (
      /*day*/
      l[12].forecast.scope == "detailed" ? 0 : (
        /*day*/
        l[12].forecast.scope == "abridged" ? 1 : (
          /*day*/
          l[12].forecast.scope == "night" ? 2 : -1
        )
      )
    );
  }
  return ~(e = u(n)) && (t = i[e] = s[e](n)), {
    c() {
      t && t.c(), r = _e();
    },
    m(l, a) {
      ~e && i[e].m(l, a), S(l, r, a), o = !0;
    },
    p(l, a) {
      let d = e;
      e = u(l), e === d ? ~e && i[e].p(l, a) : (t && (ae(), q(i[d], 1, 1, () => {
        i[d] = null;
      }), ce()), ~e ? (t = i[e], t ? t.p(l, a) : (t = i[e] = s[e](l), t.c()), C(t, 1), t.m(r.parentNode, r)) : t = null);
    },
    i(l) {
      o || (C(t), o = !0);
    },
    o(l) {
      q(t), o = !1;
    },
    d(l) {
      l && $(r), ~e && i[e].d(l);
    }
  };
}
function Qt(n) {
  let e, t, r, o, s = (
    /*report*/
    n[0].special_weather_statements.length > 0 && Fe(n)
  ), i = (
    /*ccFields*/
    n[1].length > 0 && We(n)
  ), u = (
    /*report*/
    n[0].weather_forecasts.length > 0 && Ge(n)
  );
  return {
    c() {
      s && s.c(), e = L(), i && i.c(), t = L(), u && u.c(), r = _e();
    },
    m(l, a) {
      s && s.m(l, a), S(l, e, a), i && i.m(l, a), S(l, t, a), u && u.m(l, a), S(l, r, a), o = !0;
    },
    p(l, [a]) {
      /*report*/
      l[0].special_weather_statements.length > 0 ? s ? s.p(l, a) : (s = Fe(l), s.c(), s.m(e.parentNode, e)) : s && (s.d(1), s = null), /*ccFields*/
      l[1].length > 0 ? i ? i.p(l, a) : (i = We(l), i.c(), i.m(t.parentNode, t)) : i && (i.d(1), i = null), /*report*/
      l[0].weather_forecasts.length > 0 ? u ? (u.p(l, a), a & /*report*/
      1 && C(u, 1)) : (u = Ge(l), u.c(), C(u, 1), u.m(r.parentNode, r)) : u && (ae(), q(u, 1, 1, () => {
        u = null;
      }), ce());
    },
    i(l) {
      o || (C(u), o = !0);
    },
    o(l) {
      q(u), o = !1;
    },
    d(l) {
      l && ($(e), $(t), $(r)), s && s.d(l), i && i.d(l), u && u.d(l);
    }
  };
}
function Yt(n, e, t) {
  let { report: r } = e, o = [];
  const s = (c) => `${c}`, i = (c) => (_) => `${_}${c}`, u = i("°C"), l = i("%"), a = i(" km"), d = {
    condition: s,
    temperature_c: u,
    humidity_pct: l,
    humidex_c: u,
    wind_chill_c: u,
    aqhi: s,
    dewpoint_c: u,
    pressure: (c) => {
      const _ = c.tendency ? ` ${c.tendency}` : "";
      return `${c.kpa} kPa${_}`;
    },
    visibility_km: a,
    wind: (c) => {
      let _ = "";
      return c.speed == "calm" ? _ += "Calm" : _ += `${c.speed.kph} km/h`, c.direction && (_ += ` ${c.direction}`), c.gust_kph && (_ += ` (${c.gust_kph} km/h gusts)`), _;
    }
  }, f = (c) => {
    if (c.current_conditions == null)
      return [];
    const _ = c.current_conditions, p = [];
    for (let O in d) {
      let Q = _[O];
      if (Q == null)
        continue;
      let J = d[O](Q);
      p.push([O.split("_", 2)[0], J]);
    }
    return p;
  }, h = new Intl.DateTimeFormat(
    "en-CA",
    {
      month: "short",
      day: "2-digit",
      timeZone: "UTC"
    }
  ), m = new Intl.DateTimeFormat("en-CA", { weekday: "short", timeZone: "UTC" }), k = (c) => {
    const _ = Date.parse(c), p = new Date(_);
    return {
      dayOfWeek: m.format(p),
      shortDate: h.format(p)
    };
  };
  return n.$$set = (c) => {
    "report" in c && t(0, r = c.report);
  }, n.$$.update = () => {
    n.$$.dirty & /*report*/
    1 && t(1, o = f(r));
  }, [r, o, k];
}
class Xt extends Le {
  constructor(e) {
    super(), ke(this, e, Yt, Qt, Se, { report: 0 });
  }
}
function Ze(n) {
  let e, t, r, o = {
    ctx: n,
    current: null,
    token: null,
    hasCatch: !1,
    pending: nn,
    then: tn,
    catch: en,
    value: 7,
    blocks: [, , ,]
  };
  return Ne(t = /*reportPromise*/
  n[0], o), {
    c() {
      e = _e(), o.block.c();
    },
    m(s, i) {
      S(s, e, i), o.block.m(s, o.anchor = i), o.mount = () => e.parentNode, o.anchor = e, r = !0;
    },
    p(s, i) {
      n = s, o.ctx = n, i & /*reportPromise*/
      1 && t !== (t = /*reportPromise*/
      n[0]) && Ne(t, o) || dt(o, n, i);
    },
    i(s) {
      r || (C(o.block), r = !0);
    },
    o(s) {
      for (let i = 0; i < 3; i += 1) {
        const u = o.blocks[i];
        q(u);
      }
      r = !1;
    },
    d(s) {
      s && $(e), o.block.d(s), o.token = null, o = null;
    }
  };
}
function en(n) {
  return {
    c: I,
    m: I,
    p: I,
    i: I,
    o: I,
    d: I
  };
}
function tn(n) {
  let e, t;
  return e = new Xt({ props: { report: (
    /*report*/
    n[7]
  ) } }), {
    c() {
      ie(e.$$.fragment);
    },
    m(r, o) {
      re(e, r, o), t = !0;
    },
    p(r, o) {
      const s = {};
      o & /*reportPromise*/
      1 && (s.report = /*report*/
      r[7]), e.$set(s);
    },
    i(r) {
      t || (C(e.$$.fragment, r), t = !0);
    },
    o(r) {
      q(e.$$.fragment, r), t = !1;
    },
    d(r) {
      se(e, r);
    }
  };
}
function nn(n) {
  let e;
  return {
    c() {
      e = T("Loading");
    },
    m(t, r) {
      S(t, e, r);
    },
    p: I,
    i: I,
    o: I,
    d(t) {
      t && $(e);
    }
  };
}
function rn(n) {
  let e, t, r, o, s = (
    /*reportPromise*/
    n[0] != null && Ze(n)
  );
  return {
    c() {
      e = y("div"), t = y("div"), t.innerHTML = '<input class="autocomplete-input" placeholder="Search for a city"/> <ul class="autocomplete-result-list"></ul>', r = L(), s && s.c(), E(t, "id", "location-autocomplete"), E(t, "class", "autocomplete"), E(e, "class", "try");
    },
    m(i, u) {
      S(i, e, u), b(e, t), b(e, r), s && s.m(e, null), o = !0;
    },
    p(i, [u]) {
      /*reportPromise*/
      i[0] != null ? s ? (s.p(i, u), u & /*reportPromise*/
      1 && C(s, 1)) : (s = Ze(i), s.c(), C(s, 1), s.m(e, null)) : s && (ae(), q(s, 1, 1, () => {
        s = null;
      }), ce());
    },
    i(i) {
      o || (C(s), o = !0);
    },
    o(i) {
      q(s), o = !1;
    },
    d(i) {
      i && $(e), s && s.d();
    }
  };
}
function sn(n, e, t) {
  let { apiURL: r } = e, o, s, i;
  const u = new Wt({ BASE: r }), l = (d) => u.locations.locationsSearch({ q: d }), a = (d) => u.weather.weather({
    provinceOrTerritory: d.province_or_territory,
    slug: d.slug
  });
  return ot(() => {
    o = new kt(
      "#location-autocomplete",
      {
        search(d) {
          return l(d);
        },
        getResultValue(d) {
          return d.name;
        },
        debounceTime: 200,
        onSubmit(d) {
          t(2, s = d);
        },
        autoSelect: !0
      }
    );
  }), it(() => {
    o.destroy();
  }), n.$$set = (d) => {
    "apiURL" in d && t(1, r = d.apiURL);
  }, n.$$.update = () => {
    n.$$.dirty & /*location*/
    4 && s != null && t(0, i = a(s));
  }, [i, r, s];
}
class ln extends Le {
  constructor(e) {
    super(), ke(this, e, sn, rn, Se, { apiURL: 1 });
  }
}
export {
  ln as default
};
