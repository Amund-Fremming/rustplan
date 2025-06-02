import { HttpService, urlBase } from "./httpService.js";

const modalButton = document.getElementById("modal-button");
const dialog = document.getElementById("modal");
const nameInput = document.getElementById("name-input");
const groupId = document.getElementById("group-id").innerHTML;
const linkToCopy = `${urlBase}/page/${groupId}`;
const copyButton = document.getElementById("copy");

const httpService = new HttpService();
let groupData = undefined;

window.onload = () => {
  dialog.showModal();
};

dialog.addEventListener("keydown", (event) => {
  if (event.key === "Escape") {
    event.preventDefault();
  }
});

copyButton.addEventListener("click", async () => {
  await navigator.clipboard.writeText(linkToCopy);
  alert("Lenke kopiert til utklippstavlen!");
});

modalButton?.addEventListener("click", async () => {
  console.log("Input field: ", nameInput.value);
  console.log("Group id: ", groupId);

  await httpService.joinGroup(groupId, nameInput.value);

  groupData = await httpService.getGroupData(groupId);
  dialog.close();
});

// For development only
const header = document.getElementById("header");
header.addEventListener("click", () => {
  console.log(groupData);
});
