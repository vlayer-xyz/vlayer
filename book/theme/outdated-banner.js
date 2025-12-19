// Add outdated docs banner
(function() {
    'use strict';
    
    // Create banner element
    const banner = document.createElement('div');
    banner.className = 'outdated-banner';
    banner.innerHTML = '⚠️ These docs are outdated. For the latest version, visit <a href="https://docs.vlayer.xyz" target="_blank" rel="noopener noreferrer">https://docs.vlayer.xyz</a>';
    
    // Insert banner at the top of the body
    document.body.insertBefore(banner, document.body.firstChild);
    
    // Adjust sidebar position if it exists
    const sidebar = document.querySelector('.sidebar');
    if (sidebar) {
        sidebar.style.top = '50px';
    }
})();

