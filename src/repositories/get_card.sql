SELECT
  id,
  deck_id,
  question,
  answer,
  creation_time,
  last_studied_time,
  ef,
  interval,
  queue,
  due,
  data
FROM
  cards
WHERE
  id = ?
