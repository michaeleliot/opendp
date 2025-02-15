from opendp.transformations import *
from opendp.measurements import *
from opendp.mod import enable_features

enable_features("floating-point", "contrib")


def test_dp_mean():

    data = '59,1,9,1,0,1\n31,0,1,3,17000,0\n36,1,11,1,0,1\n54,1,11,1,9100,1\n39,0,5,3,37000,0\n34,0,9,1,0,1\n'\
        '93,1,8,1,6000,1\n69,0,13,1,350000,1\n40,1,11,3,33000,1\n27,1,11,1,25000,0\n59,1,13,1,49000,1\n' \
        '31,1,11,3,0,1\n73,1,13,4,35500,0\n89,1,9,1,4000,1\n39,1,10,3,15000,0\n51,1,13,4,120000,1\n' \
        '32,0,9,1,13000,0\n52,0,11,2,45000,0\n24,0,7,1,0,0\n48,1,10,1,4300,1\n51,0,1,3,16000,1\n' \
        '43,1,14,1,365000,1\n29,0,4,3,20000,0\n44,1,15,1,17900,1\n87,1,8,1,3600,0\n27,1,11,3,10800,0\n' \
        '58,0,13,1,60900,1\n32,1,11,3,25000,1\n'

    col_names = ["A", "B", "C", "D", "E"]
    index = "E"
    impute_constant = 0.
    bounds = (0., 10000.)
    n = 1000
    scale = 1.
    preprocessor = (
        # Convert data into Vec<Vec<String>>
        make_split_dataframe(separator=",", col_names=col_names) >>
        # Selects a column of df, Vec<str>
        make_select_column(key=index, TOA=str) >>
        # Cast the column as Vec<Optional<Float>>
        make_cast(TIA=str, TOA=float) >>
        # Impute missing values to 0 Vec<Float>
        make_impute_constant(impute_constant) >>
        # Clamp values
        make_clamp(bounds) >>
        # Resize dataset length
        make_bounded_resize(n, bounds, impute_constant) >>
        # Aggregate with mean
        make_sized_bounded_mean(n, bounds) >>
        # Noise
        make_base_laplace(scale)
    )
    res = preprocessor(data)
    assert type(res) == float


