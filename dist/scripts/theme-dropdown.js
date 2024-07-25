console.log('theme-dropdown.js loaded.');
const themeBtn = document.getElementById('theme-button');
const themeDropdown = document.getElementById('theme-dropdown');

const closeThemeDropdown = function() {
    themeBtn.setAttribute('closing', '');
    themeDropdown.setAttribute('closing', 'true');
    themeDropdown.addEventListener('animationend', () => {
        themeBtn.removeAttribute('closing');
        themeBtn.removeAttribute('active');
        themeDropdown.removeAttribute('closing');
        themeDropdown.removeAttribute('active');
    }, {once: true});
}

themeBtn.addEventListener('click', () => {
    if (themeDropdown.getAttribute('active') == 'true') {
        closeThemeDropdown();
    } else {
        themeDropdown.setAttribute('active', 'true');
        themeBtn.setAttribute('opening', '');
        window.setTimeout(() => {
            window.addEventListener('click', (ev) => {
                closeThemeDropdown();
            }, {once: true});
        }, 5);
        themeDropdown.addEventListener('animationend', () => {
            themeBtn.removeAttribute('opening');
            themeBtn.setAttribute('active', '');
        });
    }
});