(ns day-4
  (:require [clojure.set :as set]
            [clojure.string :as str]))

(defn indexed [coll]
  (map-indexed vector coll))

(defn build-board [filename]
  (into #{} (for [[i line] (indexed (str/split-lines (slurp filename)))
                  [j char] (indexed line) :when (= char \@)]
              [i j])))

(defn get-neighbour-coords [[i j]]
  (for [di [-1 0 1] dj [-1 0 1]
        :when (not= 0 di dj)]
    [(+ i di) (+ j dj)]))

(defn count-neighbours [board coords]
  (count (remove nil? (for [neighbour (get-neighbour-coords coords)] (board neighbour)))))

(defn reachable? [board coords]
  (< (count-neighbours board coords) 4))

(defn all-reachable [board]
  (filter #(reachable? board %) board))

(defn clean-board [board]
  (loop [current-board board]
    (let [reachable (all-reachable current-board)]
      (if (empty? reachable)
        (- (count board) (count current-board))
        (recur (set/difference current-board reachable))))))

(comment
  (def test-board (build-board "test.txt"))
  (def real-board (build-board "input.txt"))

  (count (all-reachable test-board))
  ;; => 13
  (count (all-reachable real-board))
  ;; => 1349

  ;; part 2:
  (clean-board test-board)
  ;; => 43
  (clean-board real-board)
  ;; => 8277
  )
