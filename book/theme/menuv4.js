// make section headers in the menu clickable and expandable
document.querySelectorAll("ol.chapter> li.chapter-item").forEach((el) => {
  el.onclick = function() {
      this.classList.toggle("expanded")
  }
})

const items = document.querySelectorAll('li.chapter-item');
const target = Array.from(items).find(li => li.textContent.includes('Appendix'));
if(target && !window.location.pathname.includes("appendix")) {
  target.classList.remove('expanded');
}

// Create the sticky div with link to Discord Support
const stickyDiv = document.createElement('div');
stickyDiv.id = 'stickyBottom';
stickyDiv.innerHTML = '🛟 Need help? <a href="https://support.vlayer.xyz/" target="_blank">Discord Support</a>';

document.querySelector('.sidebar-scrollbox').appendChild(stickyDiv);
