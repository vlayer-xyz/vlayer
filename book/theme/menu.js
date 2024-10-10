// make section headers in the menu clickable and expandable
document.querySelectorAll("ol.chapter> li.chapter-item").forEach((el) => {
  el.onclick = function() {
      this.classList.toggle("expanded")
  }
})
