SELECT
	id,
  deck_id,
	question,
	answer,
	creation_time,
	last_studied_time,
	ef,
	interval,
	due,
	queue
FROM
	cards
WHERE
	deck_id = ?
	AND queue = ?
