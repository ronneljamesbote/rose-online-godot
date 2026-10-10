"use client";

// A folder's pages as a table you can filter by name and sort by any column.

import { useMemo, useState } from "react";

export interface ListCell {
  html: string;
  sort: string | number | null;
}

export interface ListRow {
  href: string;
  name: string;
  id: string;
  icon?: [string, number, number, number, number];
  status?: string;
  cells: ListCell[];
}

export default function PageList({ columns, rows }: { columns: string[]; rows: ListRow[] }) {
  const [filter, setFilter] = useState("");
  const [sort, setSort] = useState<{ col: number; desc: boolean }>({ col: -2, desc: false });

  const shown = useMemo(() => {
    const needle = filter.trim().toLowerCase();
    const list = needle ? rows.filter((r) => r.name.toLowerCase().includes(needle) || r.id === needle) : rows.slice();
    if (sort.col !== -2) {
      const key = (r: ListRow): string | number | null =>
        sort.col === -1 ? r.name.toLowerCase() : sort.col === -3 ? Number(r.id) || r.id : r.cells[sort.col]?.sort ?? null;
      list.sort((a, b) => {
        const x = key(a);
        const y = key(b);
        if (x === y) return 0;
        if (x === null) return 1;
        if (y === null) return -1;
        const order = typeof x === "number" && typeof y === "number" ? x - y : String(x).localeCompare(String(y));
        return sort.desc ? -order : order;
      });
    }
    return list;
  }, [rows, filter, sort]);

  const header = (col: number, text: string, num = false) => (
    <th className={num ? "num" : undefined}>
      <button
        type="button"
        className="sort"
        aria-sort={sort.col === col ? (sort.desc ? "descending" : "ascending") : undefined}
        onClick={() => setSort((s) => ({ col, desc: s.col === col ? !s.desc : false }))}
      >
        {text}
        {sort.col === col ? (sort.desc ? " ▼" : " ▲") : ""}
      </button>
    </th>
  );

  return (
    <div className="page-list">
      <input
        type="search"
        placeholder={`Filter ${rows.length.toLocaleString("en-US")} pages by name`}
        value={filter}
        onChange={(e) => setFilter(e.target.value)}
        aria-label="Filter by name"
      />
      <div className="table-wrap">
        <table className="wiki-table">
          <thead>
            <tr>
              {header(-3, "ID", true)}
              {header(-1, "Name")}
              {columns.map((c, i) => header(i, c, rows.some((r) => typeof r.cells[i]?.sort === "number")))}
            </tr>
          </thead>
          <tbody>
            {shown.map((r) => (
              <tr key={r.href} className={r.status === "not-in-game-yet" ? "not-in-game" : undefined}>
                <td className="num muted">{r.id}</td>
                <td>
                  {r.icon && (
                    <span
                      className="wiki-icon small"
                      aria-hidden="true"
                      style={{
                        width: r.icon[3],
                        height: r.icon[4],
                        backgroundImage: `url(/wiki-assets/icons/${r.icon[0]})`,
                        backgroundPosition: `-${r.icon[1]}px -${r.icon[2]}px`,
                      }}
                    />
                  )}
                  <a href={r.href}>{r.name}</a>
                </td>
                {r.cells.map((c, i) => (
                  <td key={i} className={typeof c.sort === "number" ? "num" : undefined} dangerouslySetInnerHTML={{ __html: c.html }} />
                ))}
              </tr>
            ))}
          </tbody>
        </table>
      </div>
      {shown.length === 0 && <p className="muted">No page matches.</p>}
    </div>
  );
}
