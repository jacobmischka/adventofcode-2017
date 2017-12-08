Imports System.Collections.Generic

Module Reallocation

	Function ReallocateMemory(registersStr As String) As String
		Dim registers() As Integer = GetRegisters(registersStr)
		
		Dim max As Integer = -1
		Dim maxIndex As Integer = -1
		For i As Integer = 0 To registers.GetUpperBound(0)
			Dim register As Integer = registers(i)
			If register > max Then
				max = register
				maxIndex = i
			End If
		Next
		
		registers(maxIndex) = 0
		
		Dim registerIndex As Integer = maxIndex
		Do
			registerIndex += 1
			If registerIndex > registers.GetUpperBound(0) Then
				registerIndex -= (registers.GetUpperBound(0) + 1)
			End If
			registers(registerIndex) += 1
			max -= 1
		Loop While max > 0

		Return RegistersToStr(registers)
	End Function
	
	Function GetRegisters(registersStr As String) As Integer()
		Dim strRegisters() As String = Split(registersStr, ControlChars.Tab)
		Dim registers() As Integer = New Integer(strRegisters.GetUpperBound(0)) {}
		
		For i As Integer = 0 To strRegisters.GetUpperBound(0)
			registers(i) = Integer.Parse(strRegisters(i))
		Next
		
		Return registers
	End Function
	
	Function RegistersToStr(registers As Integer()) As String
		Dim strRegisters() As String = New String(registers.GetUpperBound(0)) {}
			
		For i As Integer = 0 To registers.GetUpperBound(0)
			strRegisters(i) = CStr(registers(i))
		Next
			
		Return Join(strRegisters, ControlChars.Tab)
	End Function

	Sub Part1()
		Dim regs As String = GetInput()
		Dim seenRegisters As New List(Of String)
		seenRegisters.Add(regs)
		Dim cycles As Integer = 0
		
		Do
			regs = ReallocateMemory(regs)
			cycles += 1
			
			If seenRegisters.Contains(regs) Then
				Console.WriteLine("Part 1: " & cycles)
				Return
			End If
			
			seenRegisters.Add(regs)
		Loop
	End Sub
	
	Sub Part2()
		Dim regs As String = GetInput()
		Dim seenRegisters As New List(Of String)
		Dim culprit As String = ""
		seenRegisters.Add(regs)
		Dim cycles As Integer = 0
		
		Do
			regs = ReallocateMemory(regs)
			
			If seenRegisters.Contains(regs) Then
				culprit = regs
			End If
			
			seenRegisters.Add(regs)
		Loop While culprit = ""
		
		Do
			regs = ReallocateMemory(regs)
			cycles += 1
			
			If regs = culprit Then
				Console.WriteLine("Part 2: " & cycles)
				Return
			End If
		Loop
	End Sub
	
	Function GetInput() As String
		Return My.Computer.FileSystem.ReadAllText("./input/input.txt")
	End Function

	Sub Main()
		Part1()
		Part2()
	End Sub
End Module