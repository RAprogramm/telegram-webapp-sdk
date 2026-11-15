// SPDX-FileCopyrightText: 2026 RAprogramm <andrey.rozanov.vl@gmail.com>
// SPDX-License-Identifier: MIT

import { db } from 'sdk';
import { sql } from 'sdk/db';
import { visits } from '../schema.js';

export default async function (input, ctx) {
  const userId = ctx.initData.user.id;

  const [row] = await db.insert(visits)
    .values({ userId, count: 1 })
    .onConflictDoUpdate({
      target: visits.userId,
      set: { count: sql`${visits.count} + 1` },
    })
    .returning()
    .run();

  return { count: row.count };
}
