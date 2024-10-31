// make section headers in the menu clickable and expandable
document.querySelectorAll("ol.chapter> li.chapter-item").forEach((el) => {
  el.onclick = function() {
      this.classList.toggle("expanded")
  }
})

// Hide appendx/architecture stuff from menu when not authenticated
function getCookie(name) {
  const value = `; ${document.cookie}`;
  const parts = value.split(`; ${name}=`);
  if (parts.length === 2) return parts.pop().split(';').shift();
  return null;
}

if(!getCookie('authenticated')) {
  const spacerElement = document.querySelector('.spacer');
  if (spacerElement) {
    let nextElement = spacerElement.nextElementSibling;
    for (let i = 0; i < 2 && nextElement; i++) {
      nextElement.style.display = 'none';
      nextElement = nextElement.nextElementSibling;
    }
  }
  spacerElement.style.display = 'none';
}

