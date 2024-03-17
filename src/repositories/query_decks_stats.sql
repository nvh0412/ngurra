SELECT
  deck_id,
  sum(queue = :new_queue),
  sum(queue = :learn_queue),
  sum(
    queue = :review_queue
    AND due <= :day_cutoff
  ),
  COUNT(1)
FROM
  cards
GROUP BY
  deck_id
