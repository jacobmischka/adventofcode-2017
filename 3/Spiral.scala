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

object Spiral {
	def part1(): Unit = {
		print("Part 1: ")
		println(getNumMoves(getInput.toInt))
	}

	def getNumMoves(target: Int): Int = {
		val ringMoves = (getRingWidth(target) / 2.0).round
		val ringSide = getRingSide(target)
		val sideMoves = Math.abs(getSideCenter(getRingWidth(target), ringSide) - target).toInt
		return (ringMoves + sideMoves - 1).toInt
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
		return Direction.opposite(getRingSide(pos))
	}

	def getSideNum(target: Int): Float = {
		val ringWidth = getRingWidth(target)
		val ringMax = Math.pow(ringWidth, 2).toInt
		return (ringMax - target) / (ringWidth - 1).toFloat
	}

	def getRingSide(target: Int): Direction = {
		val sideNum = getSideNum(target)
		if (sideNum <= 1)
			return Down
		if (sideNum <= 2)
			return Left
		if (sideNum <= 3)
			return Up
		return Right
	}

	def getRingWidth(target: Int): Int = {
		var ring = Math.sqrt(target.toFloat).ceil.toInt
		if (ring % 2 == 0)
			ring += 1

		return ring
	}

	def getInput(): String = {
		return Source.fromFile("./input/input.txt").mkString.trim
	}

	def main(args: Array[String]): Unit = {
		part1
	}
}
