
/*

test06cpp/test06cpp.cpp

test06/src/main.rsの検証。
Beta分布のCDFの逆関数を出してみる。

Compile options: -EHsc -utf-8 -std:c++17

*/

#include <iostream>
#include <vector>
#include <boost/math/distributions/beta.hpp>

int main( int argc, char *argv[])
{

    using namespace std;
    using namespace boost::math;

    cout << "Test for InvCDF() of Beta(alpha,beta)" << endl;
    cout << " Set p = { 0.01, 0.02, ... , 0.98, 0.99}" << endl;
    cout << " Then calculate z_hat using InvCDF()" << endl;
    cout << endl;
    cout << "C++ Version" << endl;
    cout << endl;

    cout << "alpha" << "\t" << "beta" << "\t" << "p" << "\t" << "z_hat_cpp"
         << endl;
    
    vector <double> alpha_vec = { 0.5, 1.0, 2.0};
    vector <double> beta_vec  = { 0.5, 1.0, 2.0};

    for ( const auto &alpha : alpha_vec){
        for ( const auto &beta : beta_vec){

            beta_distribution <double> dist_obj( alpha, beta);

            for ( int pby100 = 1; pby100 < 100; ++pby100){

                double p = ( double)pby100 / 100.0;
                double z_hat = quantile( dist_obj, p);
                cout << alpha << "\t" << beta << "\t"
                     << setprecision( 17) 
                     << p << "\t" << z_hat << endl;

            }
            

        }
    }

}
