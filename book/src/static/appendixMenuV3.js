// Get cookie value by it's name
function getCookie(name) {
  const value = `; ${document.cookie}`;
  const parts = value.split(`; ${name}=`);
  if (parts.length === 2) return parts.pop().split(';').shift();
  return null;
}

document.addEventListener("DOMContentLoaded", function() {
  // Hide appendx/architecture stuff from menu when not authenticated.
  if(getCookie('isAuthenticated')) {
    console.log("authenticated")
    const spacerElement = document.querySelector('.spacer');
    if (spacerElement) {
      let nextElement = spacerElement.nextElementSibling;
      for (let i = 0; i < 2 && nextElement; i++) {
        nextElement.classList.add('flex');
        nextElement = nextElement.nextElementSibling;
      }
    }
  }
});
