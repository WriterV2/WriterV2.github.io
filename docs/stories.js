// every story card
const workCards = document.querySelectorAll(".work-card");

// story type select input
const storyTypeSelect = document.querySelector("#story-type-select");
// language select input
const languageSelect = document.querySelector("#language-select");
// copy of works with all work cards 
const works = Array.from(document.querySelector(".works").children);

// filter cards based on storytype and language filter
function filterWorkCards() {
    // restore initial state with all cards
    // (TODO: Explore more efficient solutions without another loop)
    document.querySelector(".works").replaceChildren();
    works.forEach((card) => {
        document.querySelector(".works").appendChild(card);
    })

    // filter card if language or story type filter doesn't match
    workCards.forEach((card) => {
        if (languageSelect.value != "*" && card.querySelector(".story-language").innerHTML != languageSelect.value) {
            document.querySelector(".works").removeChild(card);
        } else if (storyTypeSelect.value != "*" && card.querySelector(".story-type").innerHTML != storyTypeSelect.value) {
            document.querySelector(".works").removeChild(card);
        }
    })
}

// filter cards when
// story type selection changes
storyTypeSelect.addEventListener("change", filterWorkCards);
// language selection changes
languageSelect.addEventListener("change", filterWorkCards);
