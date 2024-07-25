window.setTimeout(() => {
    const subgenreBtn = document.getElementById("subgenre-button");
    const subgenreDropdown = document.getElementById("subgenre-dropdown");
    
    const closeSubgenreDropdown = function(event) {
        const eventTarget = event.target;
        if (!subgenreBtn.contains(eventTarget) && !(subgenreBtn === eventTarget))  {
            console.log("removing active attribute");
            subgenreDropdown.style.maxHeight = "0rem";
            subgenreDropdown.style.opacity = "0";
            subgenreBtn.removeAttribute("active");
            window.setTimeout(() => {
                subgenreDropdown.style.display = "none";
                subgenreDropdown.removeAttribute("active");
            }, 195);
            window.removeEventListener('click', closeSubgenreDropdown);
        }
    }

    subgenreBtn.addEventListener("click", () => {
        if (subgenreDropdown.getAttribute("active")) {
            subgenreDropdown.style.maxHeight = "0rem";
            subgenreDropdown.style.opacity = "0";
            subgenreBtn.removeAttribute("active");
            window.setTimeout(() => {
                subgenreDropdown.style.display = "none";
                subgenreDropdown.removeAttribute("active");
            }, 195);
        } else {
            subgenreDropdown.style.display = "flex";
            subgenreBtn.setAttribute("active", "true");
            window.setTimeout(() => {
                subgenreDropdown.style.maxHeight = "80vh";
                subgenreDropdown.style.opacity = "1";
            }, 5);
            window.setTimeout(() => {
                subgenreDropdown.setAttribute("active", "true");
            }, 195);
            window.addEventListener('click', closeSubgenreDropdown);
        }
    });
    
}, 100);