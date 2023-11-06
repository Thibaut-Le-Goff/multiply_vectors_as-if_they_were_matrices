 <center><h2><ins>Multiply vectors as if they were matrices</ins></h2></center>

The aim of this project is to be able to do the multiplication of two matrices, but from two vectors. 

The motivation behind this is to make the gradient descent methods (from this project) work with matrices to train multiple neurons on the same layer.

<h3><ins>The problem.</ins></h3>

The function created takes two vectors as inputs :\
vector1 = $\begin{bmatrix}
0 & 1 & 2 & 3 & 4 & 5
\end{bmatrix}$\
vector2 = $\begin{bmatrix}
0 & 1 & 2 & 3 & 4 & 5
\end{bmatrix}$

If we wanted to manipulate these vectors as matrices, we would have to rearrange them, but there are many ways of doing this:

| | The possibilities :
| --------------|---------------|
| possibility1 = $\begin{matrix}[0 & 1 & 2 & 3 & 4 & 5]\end{matrix}$ | possibility2 = $\begin{matrix}[0 &1 & 2\\ 3 & 4 & 5]\end{matrix}$ |
| possibility3 = $\begin{matrix}[0 & 1\\ 2 & 3\\ 4 & 5]\end{matrix}$ | possibility4 = $\begin{matrix}[0 \\ 1\\ 2 \\ 3\\ 4 \\ 5]\end{matrix}$ |

We can't choose one of them without informations about the number of rows/columns we want for the result matrix.

We, at least, need to either the rows or the columns of the matrix we would have as the result, this the third argument of the function.



<h3><ins>Step 1 : Extract the rows from mat1</ins></h3>

<h3><ins>Step 2 : Extract the collumns from mat1</ins></h3>
<h3><ins>Step : </ins></h3>
<h3><ins>Step : </ins></h3>
<h3><ins>Step : </ins></h3>


$$\begin{bmatrix}
1 & 2 & 3\\
a & b & c
\end{bmatrix}$$
