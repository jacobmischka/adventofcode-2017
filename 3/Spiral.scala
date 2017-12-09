import scala.collection.mutable.HashMap
import scala.io.Source
import java.lang.Math

object Direction extends Enumeration {
	type Direction = Value
	val Up, Down, Left, Right = Value

	val opposite: Direction => Direction = {
		case Up => Down
		case Left => Right
		case Down => Up
		case Right => Left
	}
}

import Direction._

class Grid {
	val contents = new HashMap[Int, HashMap[Int, Int]]()
	var radius = 0
	var x = 0
	var y = 0
	var nextDirection = Right

	set(0, 0, 1)

	def set(x: Int, y: Int, value: Int): Unit = {
		val col = contents.get(x) match {
			case Some(col) => col
			case None => {
				new HashMap[Int, Int]()
			}
		}
		col.update(y, value)
		contents.update(x, col)
	}

	def get(x: Int, y: Int): Int = {
		contents.get(x) match {
			case Some(col) => {
				col.get(y) match {
					case Some(value) => value
					case None => 0
				}
			}
			case None => 0
		}
	}

	def addRing(): Unit = {
		radius += 1
		nextDirection = Up
	}

	def posString(): String = {
		"(" + x + ", " + y + ")"
	}

	def move(direction: Direction): Unit = {
		direction match {
			case Right => {
				x += 1
				if (x == radius + 1) {
					addRing
				}
			}
			case Up => {
				y += 1
				if (y == radius) {
					nextDirection = Left
				}
			}
			case Left => {
				x -= 1
				if (x == radius * -1) {
					nextDirection = Down
				}
			}
			case Down => {
				y -= 1
				if (y == radius * -1) {
					nextDirection = Right
				}
			}
		}
	}

	def adjacentSum(): Int = {
		var sum = 0

		sum += get(x, y + 1)
		sum += get(x + 1, y + 1)
		sum += get(x + 1, y)
		sum += get(x + 1, y - 1)
		sum += get(x, y - 1)
		sum += get(x - 1, y - 1)
		sum += get(x - 1, y)
		sum += get(x - 1, y + 1)

		sum
	}

	def add(): Int = {
		move(nextDirection)
		val value = adjacentSum
		set(x, y, value)
		value
	}
}

object Spiral {
	def part1(): Unit = {
		print("Part 1: ")
		println(getNumMoves(getInput.toInt))
	}

	def part2(): Unit = {
		val target = getInput.toInt
		val grid = new Grid()

		var addedVal = 0
		while (addedVal < target) {
			addedVal = grid.add
		}

		print("Part 2: ")
		println(addedVal)
	}

	def getNumMoves(target: Int): Int = {
		val ringMoves = (getRingWidth(target) / 2.0).round
		val ringSide = getRingSide(target)
		val sideMoves = Math.abs(getSideCenter(getRingWidth(target), ringSide) - target).toInt

		(ringMoves + sideMoves - 1).toInt
	}

	def getSideCenter(ringWidth: Int, side: Direction): Int = {
		val ringMax = Math.pow(ringWidth, 2).toInt
		val bottomCenter = ringMax - (ringWidth / 2)
		return side match {
			case Down => bottomCenter
			case Left => bottomCenter - (ringWidth - 1)
			case Up => bottomCenter - ((ringWidth - 1) * 2)
			case Right => bottomCenter - ((ringWidth - 1) * 3)
		}
	}

	def getMoveToCenter(pos: Int): Direction = {
		Direction.opposite(getRingSide(pos))
	}

	def getSideNum(target: Int): Float = {
		val ringWidth = getRingWidth(target)
		val ringMax = Math.pow(ringWidth, 2).toInt

		(ringMax - target) / (ringWidth - 1).toFloat
	}

	def getRingSide(target: Int): Direction = {
		val sideNum = getSideNum(target)
		if (sideNum <= 1)
			return Down
		if (sideNum <= 2)
			return Left
		if (sideNum <= 3)
			return Up
		Right
	}

	def getRingWidth(target: Int): Int = {
		var ring = Math.sqrt(target.toFloat).ceil.toInt
		if (ring % 2 == 0)
			ring += 1

		ring
	}

	def getInput(): String = {
		Source.fromFile("./input/input.txt").mkString.trim
	}

	def main(args: Array[String]): Unit = {
		part1
		part2
	}
}
