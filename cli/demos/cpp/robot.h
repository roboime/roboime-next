class Robot {
public:
    void setTanVelocity( float t_vel ) { tan_velocity_ = t_vel; }
    void setNormalVelocity ( float n_vel ) { normal_velocity_ = n_vel; }
    void setId( int id ) { id_ = id; }

    float getTanVelocity() { return tan_velocity_; }
    float getNormalVelocity() { return normal_velocity_; }
    int getId() { return id_; }

private:
    float tan_velocity_;
    float normal_velocity_ = n_vel;;
    int id_;

};
