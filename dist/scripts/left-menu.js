console.log('left-menu.js loaded.');
const leftMenuBtn = document.getElementById('left-menu-button');
const leftMenu = document.getElementById('left-menu');
const leftMenuBg = document.getElementById('left-menu-bg');
const leftMenuCloseBtn = document.getElementById('left-menu-close-btn');

const closeLeftMenu = function() {
    leftMenu.setAttribute('closing', '');
    leftMenuBg.setAttribute('closing', '');
    leftMenu.addEventListener('animationend', () => {
        leftMenu.removeAttribute('closing');
        leftMenuBg.removeAttribute('closing');
        leftMenu.removeAttribute('active');
        leftMenuBg.removeAttribute('active');
    }, {once: true});
}

leftMenuCloseBtn.addEventListener('click', () => {
    leftMenu.setAttribute('closing', '');
    leftMenuBg.setAttribute('closing', '');
    leftMenu.addEventListener('animationend', () => {
        leftMenu.removeAttribute('closing');
        leftMenuBg.removeAttribute('closing');
        leftMenu.removeAttribute('active');
        leftMenuBg.removeAttribute('active');
    }, {once: true});
});

leftMenuBtn.addEventListener('click', () => {
    console.log('left-menu-button clicked.');
    if (leftMenu.getAttribute('active') == 'true') {
        leftMenu.setAttribute('closing', '');
        leftMenuBg.setAttribute('closing', '');
        leftMenu.addEventListener('animationend', () => {
            leftMenu.removeAttribute('closing');
            leftMenuBg.removeAttribute('closing');
            leftMenu.removeAttribute('active');
            leftMenuBg.removeAttribute('active');
        }, {once: true});
    } else {
        leftMenu.setAttribute('opening', '');
        leftMenuBg.setAttribute('opening', '');
        window.setTimeout(() => {
            leftMenu.setAttribute('active', 'true');
            leftMenuBg.setAttribute('active', '');
            leftMenuBg.addEventListener('click', () => {
                leftMenu.setAttribute('closing', '');
                leftMenuBg.setAttribute('closing', '');
                leftMenu.addEventListener('animationend', () => {
                    leftMenu.removeAttribute('closing');
                    leftMenuBg.removeAttribute('closing');
                    leftMenu.removeAttribute('active');
                    leftMenuBg.removeAttribute('active');
                }, {once: true});
            });
        }, 5);
        leftMenu.addEventListener('animationend', () => {
            leftMenu.removeAttribute('opening');
            leftMenuBg.removeAttribute('opening');
        }, {once: true});
    }
});