
/*

test04cpp/test04cpp.cpp

test04/src/main.rsの検証。
Beta分布のCDFの逆関数を出してみる。

Beta( 0.5, 0.5)のInvCDF(0.01)は、
Rustだと
　0.000274658203125
C++ (Boost)だと
　0.00024672
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

    cout << "z = InvCDF(p) for Beta(alpha,beta)" << endl;
    cout << "C++ Version" << endl;

    cout << "alpha" << "\t" << "beta" << "\t"
         << "p" << "\t" << "z" << "\t" << "calculated_p" << "\t" << "abserr"
         << endl;
    
    vector <double> alpha_vec = { 0.5, 1.0, 1.5, 2.0};
    vector <double> beta_vec  = { 0.5, 1.0, 1.5, 2.0};

    for ( const auto &alpha : alpha_vec){
        for ( const auto &beta : beta_vec){

            beta_distribution <double> dist_obj( alpha, beta);

            for ( int pby100 = 1; pby100 < 100; ++pby100){

                double p = ( double)pby100 / 100.0;
                double z = quantile( dist_obj, p);
                double calculated_p = cdf( dist_obj, z); 
                double abserr = abs( p - calculated_p);
                cout << alpha << "\t" << beta << "\t"
                     << p << "\t" << z << "\t" << calculated_p << "\t" << abserr
                     << endl;

            }
            

        }
    }

}
