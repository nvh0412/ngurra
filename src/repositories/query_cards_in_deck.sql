SELECT
  id,
  question,
  answer,
  creation_time,
  last_studied_time,
  ef,
  interval,
  due,
  queue,
  data
FROM
  cards
WHERE
  deck_id = ?
