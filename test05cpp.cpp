
/*

test05cpp/test05cpp.cpp

test05/src/main.rsの検証。
Beta分布のCDFの逆関数を出してみる。

このプログラムでの、pとp_hatの絶対誤差の最大値：
　2.44249065417534E-15

Beta( 0.5, 0.5)のInvCDF(0.01)は、
Rust (crate: statr) だと
　0.000274658203125
Rust (crate: probability) だと
　0.00024671981713422173
C++ (Boost)だと
　0.00024671981713422146
Rだと
　> qbeta( 0.01, 0.5, 0.5)
　[1] 0.0002467198
Excelだと
　=BETA.INV(0.01,0.5,0.5)
　0.00024672

*/

#include <iostream>
#include <vector>
#include <boost/math/distributions/beta.hpp>

int main( int argc, char *argv[])
{

    using namespace std;
    using namespace boost::math;

    cout << "Test for InvCDF() of Beta(alpha,beta)" << endl;
    cout << " Assume p = CDF(z)" << endl;
    cout << " First, set p, and then calculate z_hat using InvCDF()" << endl;
    cout << " After that, calculate p_hat = CDF(z_hat) for validation" << endl;
    cout << endl;
    cout << "C++ Version" << endl;
    cout << endl;

    cout << "alpha" << "\t" << "beta" << "\t"
         << "p" << "\t" << "z_hat" << "\t" << "p_hat" << "\t" << "abserr"
         << endl;
    
    vector <double> alpha_vec = { 0.5, 1.0, 1.5, 2.0};
    vector <double> beta_vec  = { 0.5, 1.0, 1.5, 2.0};

    for ( const auto &alpha : alpha_vec){
        for ( const auto &beta : beta_vec){

            beta_distribution <double> dist_obj( alpha, beta);

            for ( int pby100 = 1; pby100 < 100; ++pby100){

                double p = ( double)pby100 / 100.0;
                double z_hat = quantile( dist_obj, p);
                double p_hat = cdf( dist_obj, z_hat); 
                double abserr = abs( p - p_hat);
                cout << alpha << "\t" << beta << "\t"
                     << setprecision( 17) 
                     << p << "\t" << z_hat << "\t" << p_hat << "\t" << abserr
                     << endl;

            }
            

        }
    }

}
