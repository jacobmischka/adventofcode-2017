Imports System.Collections.Generic

Module Reallocation

	Function ReallocateMemory(registersStr As String) As String
		Dim strRegisters() As String = Split(registersStr, ControlChars.Tab)
		Dim registers() As Integer = New Integer(strRegisters.GetUpperBound(0)) {}
		
		For i As Integer = 0 To strRegisters.GetUpperBound(0)
			registers(i) = Integer.Parse(strRegisters(i))
		Next
		
		Dim max As Integer = 0
		Dim maxIndex As Integer = 0
		For i As Integer = 0 To registers.GetUpperBound(0)
			Dim register As Integer = registers(i)
			If register > max Then
				max = register
				maxIndex = i
			End If
		Next
		
		Console.WriteLine(maxIndex)
		
		For i As Integer = 0 To registers.GetUpperBound(0)
			strRegisters(i) = CStr(registers(i))
		Next
		
		Return Join(strRegisters, " ")
	End Function

	Sub Part1()
		Dim input As String = GetInput()
		Dim seenRegisters As New List(Of String)
		
		Dim newRegs As String = ReallocateMemory(input)
	End Sub
	
	Sub Part2()
		
	End Sub
	
	Function GetInput() As String
		Return My.Computer.FileSystem.ReadAllText("./input/input.txt")
	End Function

	Sub Main()
		Part1()
	End Sub
End Module