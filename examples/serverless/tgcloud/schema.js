// SPDX-FileCopyrightText: 2026 RAprogramm <andrey.rozanov.vl@gmail.com>
// SPDX-License-Identifier: MIT

import { table, integer } from 'sdk/db';

export const visits = table('visits', {
  userId: integer('user_id').primaryKey(),
  count:  integer('count').notNull().default(0),
});
