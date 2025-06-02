export class RenderService {
  createDisplay(weekNr, rows) {
    `<div>
        <h2>Week ${weekNr}</h2>
        ${rows.map(
          (row) =>
            `<div id="row">
                <p>${row.member_name}</p>
                ${row.days.map(
                  (day, index) => `<div id="${day + index}">x</div>`
                )}
            </div>`
        )}
    </div>`;
  }
}
