document.addEventListener("DOMContentLoaded", function () {
  // Load KaTeX
  const katexCSS = document.createElement("link");
  katexCSS.rel = "stylesheet";
  katexCSS.href =
    "https://cdn.jsdelivr.net/npm/katex@0.16.9/dist/katex.min.css";
  document.head.appendChild(katexCSS);

  const katexJS = document.createElement("script");
  katexJS.src = "https://cdn.jsdelivr.net/npm/katex@0.16.9/dist/katex.min.js";
  katexJS.onload = function () {
    const autoRenderJS = document.createElement("script");
    autoRenderJS.src =
      "https://cdn.jsdelivr.net/npm/katex@0.16.9/dist/contrib/auto-render.min.js";
    autoRenderJS.onload = function () {
      renderMathInElement(document.body, {
        delimiters: [
          { left: "$$", right: "$$", display: true },
          { left: "$", right: "$", display: false },
        ],
      });
    };
    document.head.appendChild(autoRenderJS);
  };
  document.head.appendChild(katexJS);
});
