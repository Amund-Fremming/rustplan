export const urlBase = "http://localhost:3000";

export class HttpService {
  async joinGroup(groupId, name) {
    const response = await fetch(`${urlBase}/groups/join`, {
      method: "POST",
      headers: {
        "Content-Type": "application/json",
      },
      body: JSON.stringify({
        name: name,
        group_id: groupId,
      }),
    });

    if (!response.ok) {
      console.error("Fetch failed with code: ", response.status);
    }

    console.log("Success joinGroup");
  }

  async getGroupData(groupId) {
    const response = await fetch(`${urlBase}/groups/${groupId}`, {
      method: "GET",
      headers: {
        "Content-Type": "application/json",
      },
    });

    if (!response.ok) {
      console.error("Fetch failed with code: ", response.status);
    }

    const data = await response.json();
    console.log("Success getGroupData");
    return data;
  }

  async createGroup(groupName, groupDesc, groupYear) {
    const response = await fetch(`${urlBase}/groups`, {
      method: "POST",
      headers: {
        "Content-Type": "application/json",
      },
      body: JSON.stringify({
        name: groupName ?? null,
        description: groupDesc,
        year: parseInt(groupYear),
      }),
    });

    if (!response.ok) {
      console.error("Fetch failed with code: ", response.status);
    }

    const data = await response.json();
    console.log("Success createGroup");
    return data;
  }
}
