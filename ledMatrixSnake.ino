#include <QueueArray.h>
/*This sketch operates a simple snake game on a 8x8 grid using an analog stick*/
//Shift register pins
int latchPin = 11;
int clockPin = 10;
int dataPin = 12;
//Ground pins for the matrix
int firstGND = 2;
int secondGND = 3;
int thirdGND = 4;
int fourthGND = 5;
int fifthGND = 6;
int sixthGND = 7;
int seventhGND = 8;
int eighthGND = 9;
//delay interval and the number of matrix refreshes before snake takes a step
int delayInterval = 100;
int scoreDisplayDelay = 1000;
int frameCounter = 0;
int frameMoveThreshold = 35;
//the abstract representation of the matrix itself
int matrix[8][8];
//Pins for the analog stick;
int XPin = A0;
int YPin = A1;
int SPin = A2;
// Variables to store analog stick readings
int Xval;
int Yval;
int Sval;
// Variables to keep track of the state of the analog stick switch
int switchStateNew;
int switchStateOld = 1; // if they show a change of the state if will indicate that the analog stick has been clicked
// Abstract variables that are used to make a representation of the snake
int snakeDirection = 1; //it keeps track of the direction of snakes movement, 1 is right, 2 is down, 3 is left and 0 is up. 4 indicates that there is no analog stick movement registered
int chosenSnakeDirection = 4;
bool isAppleActive = false; // this variable indicates whether there is an active apple somewhere on the game area
int score = 0;
bool hasJustStarted = true; // this variable will indicate that the program has just started and the player has not yet played the game
bool isGameActive = false;  // this variable stores whether or not the player is currently running the game, each click of the analog stick switch will change the value of it
int choiceDirection = 2;
int tempChoiceDirection;
bool invokeSpeedProgression = false;
// The following queues are used to store the x and y coordinates of respective segments of the snake
QueueArray<int> snakeX;
QueueArray<int> snakeY;
// The following 2D int arrays are uesd to display the number of points the player scored after the game is finished
const int8_t PROGMEM digit0[8][8] = {{0, 0, 0, 0, 0, 0, 0, 0}, {0, 0, 0, 0, 0, 0, 1, 0}, {0, 0, 0, 0, 0, 1, 0, 1}, {0, 0, 0, 0, 0, 1, 0, 1}, {0, 0, 0, 0, 0, 1, 0, 1}, {0, 0, 0, 0, 0, 1, 0, 1}, {0, 0, 0, 0, 0, 1, 0, 1}, {0, 0, 0, 0, 0, 0, 1, 0}};
const int8_t PROGMEM digit1[8][8] = {{0, 0, 0, 0, 0, 0, 0, 0}, {0, 0, 0, 0, 0, 0, 0, 1}, {0, 0, 0, 0, 0, 0, 1, 1}, {0, 0, 0, 0, 0, 1, 0, 1}, {0, 0, 0, 0, 0, 0, 0, 1}, {0, 0, 0, 0, 0, 0, 0, 1}, {0, 0, 0, 0, 0, 0, 0, 1}, {0, 0, 0, 0, 0, 0, 0, 1}};
const int8_t PROGMEM digit2[8][8] = {{0, 0, 0, 0, 0, 0, 0, 0}, {0, 0, 0, 0, 0, 0, 1, 0}, {0, 0, 0, 0, 0, 1, 0, 1}, {0, 0, 0, 0, 0, 1, 0, 1}, {0, 0, 0, 0, 0, 0, 0, 1}, {0, 0, 0, 0, 0, 0, 1, 0}, {0, 0, 0, 0, 0, 1, 0, 0}, {0, 0, 0, 0, 0, 1, 1, 1}};
const int8_t PROGMEM digit3[8][8] = {{0, 0, 0, 0, 0, 0, 0, 0}, {0, 0, 0, 0, 0, 0, 1, 0}, {0, 0, 0, 0, 0, 1, 0, 1}, {0, 0, 0, 0, 0, 0, 0, 1}, {0, 0, 0, 0, 0, 0, 1, 1}, {0, 0, 0, 0, 0, 0, 0, 1}, {0, 0, 0, 0, 0, 1, 0, 1}, {0, 0, 0, 0, 0, 0, 1, 0}};
const int8_t PROGMEM digit4[8][8] = {{0, 0, 0, 0, 0, 0, 0, 0}, {0, 0, 0, 0, 0, 0, 0, 1}, {0, 0, 0, 0, 0, 0, 1, 1}, {0, 0, 0, 0, 0, 1, 0, 1}, {0, 0, 0, 0, 0, 1, 1, 1}, {0, 0, 0, 0, 0, 0, 0, 1}, {0, 0, 0, 0, 0, 0, 0, 1}, {0, 0, 0, 0, 0, 0, 0, 1}};
const int8_t PROGMEM digit5[8][8] = {{0, 0, 0, 0, 0, 0, 0, 0}, {0, 0, 0, 0, 0, 1, 1, 1}, {0, 0, 0, 0, 0, 1, 0, 0}, {0, 0, 0, 0, 0, 1, 0, 0}, {0, 0, 0, 0, 0, 0, 1, 0}, {0, 0, 0, 0, 0, 0, 0, 1}, {0, 0, 0, 0, 0, 1, 0, 1}, {0, 0, 0, 0, 0, 0, 1, 0}};
const int8_t PROGMEM digit6[8][8] = {{0, 0, 0, 0, 0, 0, 0, 0}, {0, 0, 0, 0, 0, 0, 1, 0}, {0, 0, 0, 0, 0, 1, 0, 1}, {0, 0, 0, 0, 0, 1, 0, 0}, {0, 0, 0, 0, 0, 1, 1, 0}, {0, 0, 0, 0, 0, 1, 0, 1}, {0, 0, 0, 0, 0, 1, 0, 1}, {0, 0, 0, 0, 0, 0, 1, 0}};
const int8_t PROGMEM digit7[8][8] = {{0, 0, 0, 0, 0, 0, 0, 0}, {0, 0, 0, 0, 0, 1, 1, 1}, {0, 0, 0, 0, 0, 0, 0, 1}, {0, 0, 0, 0, 0, 0, 0, 1}, {0, 0, 0, 0, 0, 0, 1, 0}, {0, 0, 0, 0, 0, 1, 0, 0}, {0, 0, 0, 0, 0, 1, 0, 0}, {0, 0, 0, 0, 0, 1, 0, 0}};
const int8_t PROGMEM digit8[8][8] = {{0, 0, 0, 0, 0, 0, 0, 0}, {0, 0, 0, 0, 0, 0, 1, 0}, {0, 0, 0, 0, 0, 1, 0, 1}, {0, 0, 0, 0, 0, 1, 0, 1}, {0, 0, 0, 0, 0, 0, 1, 0}, {0, 0, 0, 0, 0, 1, 0, 1}, {0, 0, 0, 0, 0, 1, 0, 1}, {0, 0, 0, 0, 0, 0, 1, 0}};
const int8_t PROGMEM digit9[8][8] = {{0, 0, 0, 0, 0, 0, 0, 0}, {0, 0, 0, 0, 0, 0, 1, 0}, {0, 0, 0, 0, 0, 1, 0, 1}, {0, 0, 0, 0, 0, 1, 0, 1}, {0, 0, 0, 0, 0, 0, 1, 1}, {0, 0, 0, 0, 0, 0, 0, 1}, {0, 0, 0, 0, 0, 1, 0, 1}, {0, 0, 0, 0, 0, 0, 1, 0}};

const int8_t PROGMEM tens0[8][8] = {{0, 0, 0, 0, 0, 0, 0, 0}, {0, 0, 1, 0, 0, 0, 0, 0}, {0, 1, 0, 1, 0, 0, 0, 0}, {0, 1, 0, 1, 0, 0, 0, 0}, {0, 1, 0, 1, 0, 0, 0, 0}, {0, 1, 0, 1, 0, 0, 0, 0}, {0, 1, 0, 1, 0, 0, 0, 0}, {0, 0, 1, 0, 0, 0, 0, 0}};
const int8_t PROGMEM tens1[8][8] = {{0, 0, 0, 0, 0, 0, 0, 0}, {0, 0, 0, 1, 0, 0, 0, 0}, {0, 0, 1, 1, 0, 0, 0, 0}, {0, 1, 0, 1, 0, 0, 0, 0}, {0, 0, 0, 1, 0, 0, 0, 0}, {0, 0, 0, 1, 0, 0, 0, 0}, {0, 0, 0, 1, 0, 0, 0, 0}, {0, 0, 0, 1, 0, 0, 0, 0}};
const int8_t PROGMEM tens2[8][8] = {{0, 0, 0, 0, 0, 0, 0, 0}, {0, 0, 1, 0, 0, 0, 0, 0}, {0, 1, 0, 1, 0, 0, 0, 0}, {0, 1, 0, 1, 0, 0, 0, 0}, {0, 0, 0, 1, 0, 0, 0, 0}, {0, 0, 1, 0, 0, 0, 0, 0}, {0, 1, 0, 0, 0, 0, 0, 0}, {0, 1, 1, 1, 0, 0, 0, 0}};
const int8_t PROGMEM tens3[8][8] = {{0, 0, 0, 0, 0, 0, 0, 0}, {0, 0, 1, 0, 0, 0, 0, 0}, {0, 1, 0, 1, 0, 0, 0, 0}, {0, 0, 0, 1, 0, 0, 0, 0}, {0, 0, 1, 1, 0, 0, 0, 0}, {0, 0, 0, 1, 0, 0, 0, 0}, {0, 1, 0, 1, 0, 0, 0, 0}, {0, 0, 1, 0, 0, 0, 0, 0}};
const int8_t PROGMEM tens4[8][8] = {{0, 0, 0, 0, 0, 0, 0, 0}, {0, 0, 0, 1, 0, 0, 0, 0}, {0, 0, 1, 1, 0, 0, 0, 0}, {0, 1, 0, 1, 0, 0, 0, 0}, {0, 1, 1, 1, 0, 0, 0, 0}, {0, 0, 0, 1, 0, 0, 0, 0}, {0, 0, 0, 1, 0, 0, 0, 0}, {0, 0, 0, 1, 0, 0, 0, 0}};
const int8_t PROGMEM tens5[8][8] = {{0, 0, 0, 0, 0, 0, 0, 0}, {0, 1, 1, 1, 0, 0, 0, 0}, {0, 1, 0, 0, 0, 0, 0, 0}, {0, 1, 0, 0, 0, 0, 0, 0}, {0, 0, 1, 0, 0, 0, 0, 0}, {0, 0, 0, 1, 0, 0, 0, 0}, {0, 1, 0, 1, 0, 0, 0, 0}, {0, 0, 1, 0, 0, 0, 0, 0}};
const int8_t PROGMEM tens6[8][8] = {{0, 0, 0, 0, 0, 0, 0, 0}, {0, 0, 1, 0, 0, 0, 0, 0}, {0, 1, 0, 1, 0, 0, 0, 0}, {0, 1, 0, 0, 0, 0, 0, 0}, {0, 1, 1, 0, 0, 0, 0, 0}, {0, 1, 0, 1, 0, 0, 0, 0}, {0, 1, 0, 1, 0, 0, 0, 0}, {0, 0, 1, 0, 0, 0, 0, 0}};
const int8_t PROGMEM tens7[8][8] = {{0, 0, 0, 0, 0, 0, 0, 0}, {0, 1, 1, 1, 0, 0, 0, 0}, {0, 0, 0, 1, 0, 0, 0, 0}, {0, 0, 0, 1, 0, 0, 0, 0}, {0, 0, 1, 0, 0, 0, 0, 0}, {0, 1, 0, 0, 0, 0, 0, 0}, {0, 1, 0, 0, 0, 0, 0, 0}, {0, 1, 0, 0, 0, 0, 0, 0}};
const int8_t PROGMEM tens8[8][8] = {{0, 0, 0, 0, 0, 0, 0, 0}, {0, 0, 1, 0, 0, 0, 0, 0}, {0, 1, 0, 1, 0, 0, 0, 0}, {0, 1, 0, 1, 0, 0, 0, 0}, {0, 0, 1, 0, 0, 0, 0, 0}, {0, 1, 0, 1, 0, 0, 0, 0}, {0, 1, 0, 1, 0, 0, 0, 0}, {0, 0, 1, 0, 0, 0, 0, 0}};
const int8_t PROGMEM tens9[8][8] = {{0, 0, 0, 0, 0, 0, 0, 0}, {0, 0, 1, 0, 0, 0, 0, 0}, {0, 1, 0, 1, 0, 0, 0, 0}, {0, 1, 0, 1, 0, 0, 0, 0}, {0, 0, 1, 1, 0, 0, 0, 0}, {0, 0, 0, 1, 0, 0, 0, 0}, {0, 1, 0, 1, 0, 0, 0, 0}, {0, 0, 1, 0, 0, 0, 0, 0}};

const int8_t PROGMEM s[8][8] = {{0, 0, 0, 0, 0, 0, 0, 0}, {0, 0, 1, 0, 0, 0, 0, 0}, {0, 1, 0, 1, 0, 0, 0, 0}, {0, 1, 0, 0, 0, 0, 0, 0}, {0, 0, 1, 0, 0, 0, 0, 0}, {0, 0, 0, 1, 0, 0, 0, 0}, {0, 1, 0, 1, 0, 0, 0, 0}, {0, 0, 1, 0, 0, 0, 0, 0}};
const int8_t PROGMEM cross[8][8] = {{0, 0, 0, 0, 0, 0, 0, 0}, {0, 0, 0, 0, 1, 0, 0, 0}, {0, 0, 0, 1, 1, 1, 0, 0}, {0, 0, 1, 0, 1, 0, 1, 0}, {0, 1, 1, 1, 1, 1, 1, 1}, {0, 0, 1, 0, 1, 0, 1, 0}, {0, 0, 0, 1, 1, 1, 0, 0}, {0, 0, 0, 0, 1, 0, 0, 0}};
const int8_t PROGMEM cup[8][8] = {{0, 1, 1, 1, 1, 1, 1, 1}, {0, 1, 1, 1, 0, 1, 1, 1}, {0, 1, 1, 0, 0, 1, 1, 1}, {0, 0, 1, 1, 0, 1, 1, 0}, {0, 0, 1, 1, 0, 1, 1, 0}, {0, 0, 0, 1, 1, 1, 0, 0}, {0, 0, 0, 0, 1, 0, 0, 0}, {0, 0, 1, 1, 1, 1, 1, 0}};
// This function returns a direction of the snake based on two readings of the potentiometers in the analog stick
int stickDirection(int Xval, int Yval)
{
    if (Xval == 0)
    {
        return 0; // condition for up
    }
    if (Yval == 0)
    {
        return 1; // condition for right
    }
    if (Xval > 1000)
    {
        return 2; // condition for down
    }
    if (Yval > 1000)
    {
        return 3; // condition for left
    }
    return 4; // stick has not been touched
}
void lightMatrix(const int8_t matrix[8][8])
{
    int shiftRegisterInput = 128;
    for (int i = 0; i < 8; i++) //for each of eight columns the function checks which diodes of the row need to be lighted up and then sets their coordinates to ground
    {

        digitalWrite(latchPin, LOW);
        shiftOut(dataPin, clockPin, LSBFIRST, shiftRegisterInput);
        digitalWrite(latchPin, HIGH);
        for (int j = 0; j < 8; j++)
        {
            if (pgm_read_byte_near(&matrix[j][i]) == 1 || pgm_read_byte_near(&matrix[j][i]) == 2) // 1 stands for a snake segment and 2 for an apple
            {
                switch (j)
                {
                case 0:
                    digitalWrite(firstGND, LOW);
                    break;
                case 1:
                    digitalWrite(secondGND, LOW);
                    break;
                case 2:
                    digitalWrite(thirdGND, LOW);
                    break;
                case 3:
                    digitalWrite(fourthGND, LOW);
                    break;
                case 4:
                    digitalWrite(fifthGND, LOW);
                    break;
                case 5:
                    digitalWrite(sixthGND, LOW);
                    break;
                case 6:
                    digitalWrite(seventhGND, LOW);
                    break;
                case 7:
                    digitalWrite(eighthGND, LOW);
                    break;
                }
            }
            delayMicroseconds(delayInterval); // this delay is necessary and will determine the refresh rate of the matrix display
            // After each iteration the ground needs to be disconnected
            digitalWrite(firstGND, HIGH);
            digitalWrite(secondGND, HIGH);
            digitalWrite(thirdGND, HIGH);
            digitalWrite(fourthGND, HIGH);
            digitalWrite(fifthGND, HIGH);
            digitalWrite(sixthGND, HIGH);
            digitalWrite(seventhGND, HIGH);
            digitalWrite(eighthGND, HIGH);
        }

        shiftRegisterInput /= 2;
    }
}

// This function lights respective diodes on the 8x8 led matrix
void lightMatrix(int matrix[8][8])
{
    int shiftRegisterInput = 128;
    for (int i = 0; i < 8; i++) //for each of eight columns the function checks which diodes of the row need to be lighted up and then sets their coordinates to ground
    {

        digitalWrite(latchPin, LOW);
        shiftOut(dataPin, clockPin, LSBFIRST, shiftRegisterInput);
        digitalWrite(latchPin, HIGH);
        for (int j = 0; j < 8; j++)
        {
            if (matrix[j][i] == 1 || matrix[j][i] == 2) // 1 stands for a snake segment and 2 for an apple
            {
                switch (j)
                {
                case 0:
                    digitalWrite(firstGND, LOW);
                    break;
                case 1:
                    digitalWrite(secondGND, LOW);
                    break;
                case 2:
                    digitalWrite(thirdGND, LOW);
                    break;
                case 3:
                    digitalWrite(fourthGND, LOW);
                    break;
                case 4:
                    digitalWrite(fifthGND, LOW);
                    break;
                case 5:
                    digitalWrite(sixthGND, LOW);
                    break;
                case 6:
                    digitalWrite(seventhGND, LOW);
                    break;
                case 7:
                    digitalWrite(eighthGND, LOW);
                    break;
                }
            }
            delayMicroseconds(delayInterval); // this delay is necessary and will determine the refresh rate of the matrix display
            // After each iteration the ground needs to be disconnected
            digitalWrite(firstGND, HIGH);
            digitalWrite(secondGND, HIGH);
            digitalWrite(thirdGND, HIGH);
            digitalWrite(fourthGND, HIGH);
            digitalWrite(fifthGND, HIGH);
            digitalWrite(sixthGND, HIGH);
            digitalWrite(seventhGND, HIGH);
            digitalWrite(eighthGND, HIGH);
        }

        shiftRegisterInput /= 2;
    }
}
// The following methods are responsible for the abstract operation of the game
// This function adds the coordinates of a new snake segment to both queues and also changes the state of the respective place on the matrix
void enqueueSnakeSegment(int x, int y)
{
    snakeX.enqueue(x);
    snakeY.enqueue(y);
    matrix[y][x] = 1;
}
// This function earses the last segment of the snake from the matrix whenever it moves 1 place forward. If an apple is consumed, this function is skipped
void dequeueSnakeSegment()
{
    int x = snakeX.dequeue();
    int y = snakeY.dequeue();
    matrix[y][x] = 0;
}
// This function will generate a new apple at a certain free spot on the matrix
void generateApple(int matrix[8][8])
{
    bool isCorrect = false;
    while (!isCorrect)
    {
        int appleX = rand() % 7 + 1;
        int appleY = rand() % 7;
        if (matrix[appleY][appleX] == 0)
        {
            matrix[appleY][appleX] = 2;
            isCorrect = true;
        }
    }
}
void showScore(int score)// This function will display the score of the playre after each game.
{
    // First the score gets converted into respective digits
    int tens = score / 10;
    int digit = score % 10;

    if (tens == 0)
    {
        lightMatrix(tens0);
    }
    else if (tens == 1)
    {
        lightMatrix(tens1);
    }
    else if (tens == 2)
    {
        lightMatrix(tens2);
    }
    else if (tens == 3)
    {
        lightMatrix(tens3);
    }
    else if (tens == 4)
    {
        lightMatrix(tens4);
    }
    else if (tens == 5)
    {
        lightMatrix(tens5);
    }
    else if (tens == 6)
    {
        lightMatrix(tens6);
    }
    else if (tens == 7)
    {
        lightMatrix(tens7);
    }
    else if (tens == 8)
    {
        lightMatrix(tens8);
    }
    else if (tens == 9)
    {
        lightMatrix(tens9);
    }
    delayMicroseconds(scoreDisplayDelay); // initially the first digit is displayed and then the second one is displayed and they are swiched repeatedly
    if (digit == 0)
    {
        lightMatrix(digit0);
    }
    else if (digit == 1)
    {
        lightMatrix(digit1);
    }
    else if (digit == 2)
    {
        lightMatrix(digit2);
    }
    else if (digit == 3)
    {
        lightMatrix(digit3);
    }
    else if (digit == 4)
    {
        lightMatrix(digit4);
    }
    else if (digit == 5)
    {
        lightMatrix(digit5);
    }
    else if (digit == 6)
    {
        lightMatrix(digit6);
    }
    else if (digit == 7)
    {
        lightMatrix(digit7);
    }
    else if (digit == 8)
    {
        lightMatrix(digit8);
    }
    else if (digit == 9)
    {
        lightMatrix(digit9);
    }
}
void setup()
{
    // Shift register pins
    pinMode(latchPin, OUTPUT);
    pinMode(dataPin, OUTPUT);
    pinMode(clockPin, OUTPUT);

    // Ground pins for each of the ten columns of the matrix
    pinMode(firstGND, OUTPUT);
    pinMode(secondGND, OUTPUT);
    pinMode(thirdGND, OUTPUT);
    pinMode(fourthGND, OUTPUT);
    pinMode(fifthGND, OUTPUT);
    pinMode(sixthGND, OUTPUT);
    pinMode(seventhGND, OUTPUT);
    pinMode(eighthGND, OUTPUT);

    // Pins for the analog stick
    pinMode(XPin, INPUT);
    pinMode(YPin, INPUT);
    pinMode(SPin, INPUT);

    // Ground pins need to be set to high in order to "disable" them
    digitalWrite(firstGND, HIGH);
    digitalWrite(secondGND, HIGH);
    digitalWrite(thirdGND, HIGH);
    digitalWrite(fourthGND, HIGH);
    digitalWrite(fifthGND, HIGH);
    digitalWrite(sixthGND, HIGH);
    digitalWrite(seventhGND, HIGH);
    digitalWrite(eighthGND, HIGH);
    digitalWrite(SPin, HIGH);

    Serial.begin(115200); //Serial Monitor for debugging purposes
}
void loop()
{
    // At first the game is inactive and the player gets to choose the speed of the snake
    while (!isGameActive)
    {
        switchStateNew = digitalRead(SPin);
        // if the switch was clicked the game will restart
        if (switchStateNew == 0 && switchStateOld == 1)
        {
            isGameActive = true;
            isAppleActive = false;
            chosenSnakeDirection =1;
        }
        switchStateOld = switchStateNew;
        //if the game was started for the first time, the player has no score registered, therefore a "cross" icon is displayed, prompting the player to choose the speed of snakes movement
        if (hasJustStarted)
        {
            lightMatrix(cross);
        }
        else
        {
            if(score==56){
                lightMatrix(cup);
            }else{
                showScore(score);
            }
        }
        Xval = analogRead(XPin);
        Yval = analogRead(YPin);
        tempChoiceDirection = stickDirection(Xval, Yval);
        if (tempChoiceDirection != 4)
        {
            choiceDirection = tempChoiceDirection;
            switch (choiceDirection)
            {
            case 0:
                frameMoveThreshold = 35;
                invokeSpeedProgression = true;// invokes the function that will inrease the movement speed of the snake
                break;
            case 1:
                frameMoveThreshold = 35;
                break;
            case 2:
                frameMoveThreshold = 25;
                break;
            case 3:
                frameMoveThreshold = 15;
                break;
            }
            for (int i = 0; i < 50; i++)
            {
                lightMatrix(s);// displays the letter "s" denoting "speed"
                delayMicroseconds(scoreDisplayDelay);
                switch (choiceDirection)
                {
                case 0:
                    lightMatrix(digit0);
                    break;
                case 1:
                    lightMatrix(digit1);
                    break;
                case 2:
                    lightMatrix(digit2);
                    break;
                case 3:
                    lightMatrix(digit3);
                    break;
                }
            }
        }
    }
    for (int i = 0; i < 8; i++)
    {
        for (int j = 0; j < 8; j++)
        {
            matrix[i][j] = 0;
        }
    }
    // it cleans the queues after the previous game
    while (!snakeX.isEmpty())
    {
        dequeueSnakeSegment();
    }
    // initial values of the snake coordinates and direction
    int xPos = 0;
    int yPos = 0;
    snakeDirection = 1;
    score = 0;

    while (isGameActive)
    {
        frameCounter++; // at the beginning of each iteration the nummber of frames displayed is increased
        if (hasJustStarted)
        {
            hasJustStarted = false;
        }

        // if the switch was clicked the game will terminate
        switchStateNew = digitalRead(SPin);
        if (switchStateNew == 0 && switchStateOld == 1)
        {
            if (isGameActive)
            {
                isGameActive = false;
            }
            else
            {
                isGameActive = true;
                isAppleActive = false;
            }
        }
        switchStateOld = switchStateNew;

        // first reading the direction of the analog stick
        Xval = analogRead(XPin);
        Yval = analogRead(YPin);
        //Serial.println(stickDirection(Xval, Yval));
        int temporaryDirection = stickDirection(Xval, Yval); // this variable stores the direction of the snake that the player chose before the snake made a step forward
        if (temporaryDirection != 4 && (abs(temporaryDirection - snakeDirection) != 2))
        {
            chosenSnakeDirection = temporaryDirection;
        }
        // As the count of frames reaches a previously defined threshold, a snake will move 1 step forward, the higher the threshold, the slower the speed of the snake
        if (frameCounter == frameMoveThreshold)
        {
            if (chosenSnakeDirection != 4)
            {
                snakeDirection = chosenSnakeDirection;
            }
            if (score == 55)
            {
                isAppleActive = false;
                isGameActive = false;
            }
            if (isAppleActive)
            {
                dequeueSnakeSegment();
            }
            else
            {
                generateApple(matrix);
                isAppleActive = true;
            }

            if (snakeDirection == 0)
            {
                if (yPos >= 0)
                    yPos--;
            }
            else if (snakeDirection == 1)
            {
                if (xPos <= 7)
                    xPos++;
            }
            else if (snakeDirection == 2)
            {
                if (yPos <= 7)
                    yPos++;
            }
            else if (snakeDirection == 3)
            {
                if (xPos >= 1) // once again on a regular 8x8 matrix it should be 0 here
                    xPos--;
            }

            if (xPos == 0 || xPos == 8 || yPos == -1 || yPos == 8) // if the player crushes at the wall, the game will terminate
            {
                isAppleActive = false;
                isGameActive = false;
                invokeSpeedProgression = false;
            }

            if (matrix[yPos][xPos] == 2) // what happens when an apple is consumed
            {
                isAppleActive = false;
                score++;
                if (invokeSpeedProgression)
                {
                    frameMoveThreshold -= score % 2;
                }
                enqueueSnakeSegment(xPos, yPos);
            }
            else if (matrix[yPos][xPos] == 1) // if snake crushes into itself game terminates
            {
                isAppleActive = false;
                isGameActive = false;
                invokeSpeedProgression = false;
            }
            else // if matrix at the given coordinates is empty, a new segment of the snake will be added
            {
                enqueueSnakeSegment(xPos, yPos);
            }
            frameCounter = 0; // at the end of each step the frame counter resets
        }

        lightMatrix(matrix); //the generated picture is displayed
    }
}