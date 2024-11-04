// Hide appendx/architecture stuff from menu when not authenticated
function getCookie(name) {
  const value = `; ${document.cookie}`;
  const parts = value.split(`; ${name}=`);
  console.log("debug", { value, parts})
  if (parts.length === 2) return parts.pop().split(';').shift();
  return null;
}

document.addEventListener("DOMContentLoaded", function() {
  if(!getCookie('isAuthenticated')) {
    console.log("no cookie...")
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
});



