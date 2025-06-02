import { HttpService, urlBase } from "./httpService.js";

const createButton = document.getElementById("create");
const groupName = document.getElementById("group-name");
const groupDesc = document.getElementById("group-desc");
const groupYear = document.getElementById("group-year");

const httpService = new HttpService();

createButton?.addEventListener("click", async () => {
  console.log("clicked");
  const code = await httpService.createGroup(
    groupName.value,
    groupDesc.value,
    groupYear.value
  );
  const url = `${urlBase}/page/${code}`;
  window.location.href = url;
});
