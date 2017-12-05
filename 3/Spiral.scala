import scala.collection.mutable.ArrayBuffer
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

class Spiral {
	val contents = new ArrayBuffer[ArrayBuffer[Int]]()
	addRing
	
	override def toString(): String = {
		this.contents.toString()
	}
	
	def addRing(): Unit = {
		contents.append(new ArrayBuffer[Int]())
	}
	
	def add(): Int = {
		if (ringFull(currentRing, currentRingNum)) {
			addRing
		}
		
		var newVal = 0
		
		if (currentRingNum == 0 && currentRing.size == 0) {
			newVal = 1
		} else {
			newVal += peek
			
			val ringWidth = getRingWidth(currentRingNum)
			val ring = currentRing
			val ringMax = maxLength(currentRingNum)
			val prevIndex = currentIndex
			val thisIndex = prevIndex + 1
			val nextIndex = thisIndex + 1
			
			if (
				prevIndex > 0
				&& isCorner(prevIndex, ringWidth)
			) {
				newVal += ring(prevIndex - 1)
			}
			
			if (nextIndex >= ringMax) {
				newVal += ring(nextIndex % ringMax)
			}
			
			if (
				nextIndex + 1 >= ringMax
				&& isCorner(nextIndex, ringWidth)
			) {
				newVal += ring((nextIndex + 1) % ringMax)
			}
			
			val prevRingNum = currentRingNum - 1
			val prevRing = contents(prevRingNum)
			val prevRingMax = maxLength(prevRingNum)
			
			if (prevRingNum == 0) {
				newVal += prevRing(0)
			} else {
				newVal += prevRing(thisIndex)
				newVal += prevRing((thisIndex + prevRingMax - 1) % prevRingMax)
				newVal += prevRing((thisIndex + prevRingMax - 2) % prevRingMax)
			}
		}
		
		currentRing.append(newVal)
		newVal
	}
	
	def currentIndex(): Int = {
		currentRing.size - 1
	}
	
	def peek(): Int = {
		if (currentIndex < 0)
			return 0
			
		val ring = currentRing
		ring(currentIndex)
	}
	
	def currentRingNum(): Int = {
		this.contents.size - 1
	}
	
	def currentRing(): ArrayBuffer[Int] = {
		val ringNum = this.currentRingNum()
		this.contents(ringNum)
	}
		
	def getRingWidth(ringNum: Int): Int = {
		(ringNum + 1) * 2 - 1
	}
	
	def isCorner(index: Int, ringWidth: Int): Boolean = {
		((index + 1) / (ringWidth - 1).toFloat) % 1 == 0
	}
	
	def maxLength(ringNum: Int): Int = {
		if (ringNum == 0)
			return 1
			
		Math.pow(2, getRingWidth(ringNum)).toInt
	}
	
	def ringFull(ring: ArrayBuffer[Int], ringNum: Int): Boolean = {
		ring.size == maxLength(ringNum)
	}
}

object Spiral {
	def part1(): Unit = {
		print("Part 1: ")
		println(getNumMoves(getInput.toInt))
	}
	
	def part2(): Unit = {
		val spiral = new Spiral()
		var i = 0
		while (i < 12) {
			spiral.add
			i += 1
		}
		println(spiral)
		
		// print("Part 2: ")
		// println()
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
