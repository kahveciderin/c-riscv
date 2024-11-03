int fixed_point(int x, int y, int point){
    return (x * y) >> point;
}

int mul_pi_fixed_point(int number){
    int pi = 0x0003374F;
    return fixed_point(number, pi, 16);
}

int main(){
    return mul_pi_fixed_point(100);
}
