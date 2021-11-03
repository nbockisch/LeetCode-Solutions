'''
Author: Nathan Bockisch
Date: November 3, 2021
'''

# """
# This is the robot's control interface.
# You should not implement it, or speculate about its implementation
# """
#class Robot:
#    def move(self):
#        """
#        Returns true if the cell in front is open and robot moves into the cell.
#        Returns false if the cell in front is blocked and robot stays in the current cell.
#        :rtype bool
#        """
#
#    def turnLeft(self):
#        """
#        Robot will stay in the same cell after calling turnLeft/turnRight.
#        Each turn will be 90 degrees.
#        :rtype void
#        """
#
#    def turnRight(self):
#        """
#        Robot will stay in the same cell after calling turnLeft/turnRight.
#        Each turn will be 90 degrees.
#        :rtype void
#        """
#
#    def clean(self):
#        """
#        Clean the current cell.
#        :rtype void
#        """

class Solution:
    def cleanRoom(self, robot):
        """
        :type robot: Robot
        :rtype: None
        """
        
        # When we need to backtrack, reverse a space
        def reverse():
            robot.turnRight()
            robot.turnRight()
            robot.move()
            robot.turnLeft()
            robot.turnLeft()
        
        def search(x, y, dx, dy):
            # Clean current tile and make sure we don't visit it twice
            robot.clean()
            visited.add((x, y))
            
            # Clean in all four directions
            for i in range(4):
                # Go in one direction until an obstacle or visited node is found,
                # backtrack and explore the next direction
                if ((x + dx, y + dy) not in visited) and robot.move():
                    search(x + dx, y + dy, dx, dy)
                    reverse()
                
                robot.turnLeft()
                dx, dy = -dy, dx
                
        visited = set()
        search(0, 0, 0, 1)
